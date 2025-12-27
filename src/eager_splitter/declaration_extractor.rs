use quote::ToTokens;
use syn::{self, Item, spanned::Spanned};
use syn::visit::Visit;
use sha2::{Digest, Sha256};
use hex;

use crate::ExtractedDecl;
use crate::ExtractedDeclMetadata;
use crate::source_tracker::span_to_location;
use crate::extracted_decl::SourceLocation;

struct AstMetricVisitor {
    max_depth: usize,
    current_depth: usize,
    node_count: usize,
}

impl<'ast> Visit<'ast> for AstMetricVisitor {
    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        self.node_count += 1;
        self.current_depth += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
        syn::visit::visit_expr(self, i);
        self.current_depth -= 1;
    }

    fn visit_item(&mut self, i: &'ast syn::Item) {
        self.node_count += 1;
        self.current_depth += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
        syn::visit::visit_item(self, i);
        self.current_depth -= 1;
    }

    fn visit_stmt(&mut self, i: &'ast syn::Stmt) {
        self.node_count += 1;
        self.current_depth += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
        syn::visit::visit_stmt(self, i);
        self.current_depth -= 1;
    }

    fn visit_type(&mut self, i: &'ast syn::Type) {
        self.node_count += 1;
        self.current_depth += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
        syn::visit::visit_type(self, i);
        self.current_depth -= 1;
    }

    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        self.node_count += 1;
        self.current_depth += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
        syn::visit::visit_macro(self, i);
        self.current_depth -= 1;
    }

    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        self.node_count += 1;
        self.current_depth += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
        syn::visit::visit_attribute(self, i);
        self.current_depth -= 1;
    }

    // Add other relevant visit methods for more accurate metrics
}

fn calculate_ast_metrics(item: &Item) -> (usize, usize) {
    let mut visitor = AstMetricVisitor {
        max_depth: 0,
        current_depth: 0,
        node_count: 0,
    };
    visitor.visit_item(item);
    (visitor.max_depth, visitor.node_count)
}


/// Extracts a single declaration from a `syn::Item`.
/// Returns `Some(ExtractedDecl)` if the item is a supported declaration type, `None` otherwise.
pub fn extract_single_declaration(item: &Item, item_count: usize) -> Option<ExtractedDecl> {
    let content_token_stream = item.to_token_stream();
    let (ast_depth, ast_node_count) = calculate_ast_metrics(item);

    let output_hash = {
        let mut hasher = Sha256::new();
        hasher.update(content_token_stream.to_string());
        hex::encode(hasher.finalize())
    };

    let metadata = ExtractedDeclMetadata {
        ast_depth,
        ast_node_count,
        output_hash: Some(output_hash),
        inputs_hash: None, // Placeholder for future implementation
        rings_of_sizes: Vec::new(), // Placeholder for future implementation
        zkp_witness_hash: None, // Placeholder, calculated later
    };

    let extracted_decl_base = |name: String, kind: String| {
        let source_location = span_to_location(item.span());
        let mut source_map = std::collections::HashMap::new();
        // For now, map the entire declaration to its source location
        // In a more sophisticated implementation, we would map individual tokens
        source_map.insert(0, source_location);
        
        ExtractedDecl {
            name,
            kind,
            content: content_token_stream.clone(),
            metadata,
            source_map,
        }
    };

    match item {
        Item::Fn(item_fn) => Some(extracted_decl_base(item_fn.sig.ident.to_string(), "fn".to_string())),
        Item::Struct(item_struct) => Some(extracted_decl_base(item_struct.ident.to_string(), "struct".to_string())),
        Item::Enum(item_enum) => Some(extracted_decl_base(item_enum.ident.to_string(), "enum".to_string())),
        Item::Const(item_const) => Some(extracted_decl_base(item_const.ident.to_string(), "const".to_string())),
        Item::Static(item_static) => Some(extracted_decl_base(item_static.ident.to_string(), "static".to_string())),
        Item::Trait(item_trait) => Some(extracted_decl_base(item_trait.ident.to_string(), "trait".to_string())),
        Item::Impl(item_impl) => {
            let name = if let Some((_, path, _)) = item_impl.trait_.as_ref() {
                let trait_name = path.to_token_stream().to_string()
                    .replace("::", "_")
                    .replace(" ", "")
                    .replace("<", "_")
                    .replace(">", "_")
                    .replace("(", "_")
                    .replace(")", "_")
                    .replace(",", "_")
                    .replace("'", "_")
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_')
                    .collect::<String>();
                format!("impl_for_{}", trait_name)
            } else if let syn::Type::Path(type_path) = &*item_impl.self_ty {
                if let Some(segment) = type_path.path.segments.last() {
                    format!("impl_for_{}", segment.ident.to_string())
                } else {
                    format!("impl_{}", item_count)
                }
            } else {
                format!("impl_{}", item_count)
            };
            Some(extracted_decl_base(name, "impl".to_string()))
        },
        Item::Type(item_type) => Some(extracted_decl_base(item_type.ident.to_string(), "type".to_string())),
        Item::Union(item_union) => Some(extracted_decl_base(item_union.ident.to_string(), "union".to_string())),
        Item::Use(item_use) => {
            // Use statements are handled separately - this is expected
            None
        },
        Item::Macro(item_macro) => {
            let macro_name = item_macro.mac.path.segments.last()
                .map_or("unknown_macro".to_string(), |s| s.ident.to_string());
            // EMIT ALL CODE: Process macros instead of skipping
            Some(extracted_decl_base(macro_name, "macro".to_string()))
        },
        Item::Mod(item_mod) => {
            let mod_name = item_mod.ident.to_string();
            
            // If the module has content (inline module), recursively extract its items
            if let Some((_, items)) = &item_mod.content {
                // For inline modules, we should extract each item separately
                // But for now, treat as single module to avoid breaking existing logic
                Some(extracted_decl_base(mod_name, "mod".to_string()))
            } else {
                // External module (mod foo;) - treat as single declaration
                Some(extracted_decl_base(mod_name, "mod".to_string()))
            }
        }
        Item::ExternCrate(item_extern) => {
            let crate_name = item_extern.ident.to_string();
            // Handle extern crate declarations like other items
            Some(extracted_decl_base(crate_name, "extern_crate".to_string()))
        },
        _ => {
            let content = item.to_token_stream().to_string();
            // Handle unsupported items gracefully - generate generic wrapper
            eprintln!("WARNING: UNSUPPORTED ITEM TYPE: {} - generating generic wrapper", content);
            Some(extracted_decl_base("unsupported_item".to_string(), "generic".to_string()))
        }
    }
}

