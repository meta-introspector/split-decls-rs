use anyhow::Result;
use syn::{parse_quote, Item, ItemFn, ItemStruct, ItemEnum, ItemTrait, ItemImpl};
use proc_macro2::TokenStream;
use quote::quote;

/// Universal AST execution context trait
pub trait UniversalAst {
    type TokenStream;
    type Item;
    type Error;
    
    fn parse_item(&self, input: Self::TokenStream) -> Result<Self::Item, Self::Error>;
    fn transform_item(&self, item: Self::Item) -> Result<Self::Item, Self::Error>;
    fn generate_code(&self, item: Self::Item) -> Self::TokenStream;
}

/// Security and ACL trait for AST operations
pub trait SecureExecution {
    fn check_permission(&self, operation: &AstOperation) -> bool;
    fn sandbox_execute<F, T>(&self, f: F) -> Result<T, SecurityError>
    where F: FnOnce() -> T;
}

/// AST operation types for security checking
#[derive(Debug, Clone)]
pub enum AstOperation {
    ParseItem,
    TransformItem,
    GenerateCode,
    FileAccess(String),
    NetworkAccess,
    SystemCall(String),
}

#[derive(Debug)]
pub struct SecurityError(pub String);

/// Main syn2macro converter
pub struct Syn2MacroConverter {
    security: Box<dyn SecureExecution>,
}

impl Syn2MacroConverter {
    pub fn new(security: Box<dyn SecureExecution>) -> Self {
        Self { security }
    }
    
    /// Convert syn-based code to universal trait-based macro
    pub fn convert_syn_to_traits(&self, input: TokenStream) -> Result<TokenStream> {
        // Check permission
        if !self.security.check_permission(&AstOperation::ParseItem) {
            return Err(anyhow::anyhow!("Permission denied for ParseItem"));
        }
        
        // Parse the input
        let item: Item = syn::parse2(input)?;
        
        // Convert based on item type
        let trait_impl = match item {
            Item::Fn(func) => self.convert_function_to_trait(func)?,
            Item::Struct(struct_item) => self.convert_struct_to_trait(struct_item)?,
            Item::Enum(enum_item) => self.convert_enum_to_trait(enum_item)?,
            Item::Trait(trait_item) => self.convert_trait_to_universal(trait_item)?,
            Item::Impl(impl_item) => self.convert_impl_to_universal(impl_item)?,
            _ => return Err(anyhow::anyhow!("Unsupported item type")),
        };
        
        Ok(trait_impl)
    }
    
    fn convert_function_to_trait(&self, func: ItemFn) -> Result<TokenStream> {
        let func_name = &func.sig.ident;
        let inputs = &func.sig.inputs;
        let output = &func.sig.output;
        let block = &func.block;
        
        Ok(quote! {
            pub trait #func_name {
                fn execute(#inputs) #output {
                    #block
                }
            }
            
            impl<T: UniversalAst> #func_name for T {
                fn execute(#inputs) #output {
                    // Security check
                    if !self.check_permission(&AstOperation::TransformItem) {
                        panic!("Permission denied");
                    }
                    
                    // Execute in sandbox
                    self.sandbox_execute(|| {
                        #block
                    }).unwrap()
                }
            }
        })
    }
    
    fn convert_struct_to_trait(&self, struct_item: ItemStruct) -> Result<TokenStream> {
        let struct_name = &struct_item.ident;
        let fields = &struct_item.fields;
        
        Ok(quote! {
            pub trait #struct_name {
                type Fields = #fields;
                
                fn new(fields: Self::Fields) -> Self;
                fn fields(&self) -> &Self::Fields;
            }
        })
    }
    
    fn convert_enum_to_trait(&self, enum_item: ItemEnum) -> Result<TokenStream> {
        let enum_name = &enum_item.ident;
        let variants = &enum_item.variants;
        
        Ok(quote! {
            pub trait #enum_name {
                type Variants;
                
                fn match_variant(&self) -> Self::Variants;
            }
        })
    }
    
    fn convert_trait_to_universal(&self, trait_item: ItemTrait) -> Result<TokenStream> {
        let trait_name = &trait_item.ident;
        let items = &trait_item.items;
        
        Ok(quote! {
            pub trait Universal#trait_name: UniversalAst + SecureExecution {
                #(#items)*
            }
        })
    }
    
    fn convert_impl_to_universal(&self, impl_item: ItemImpl) -> Result<TokenStream> {
        let self_ty = &impl_item.self_ty;
        let items = &impl_item.items;
        
        Ok(quote! {
            impl<T: UniversalAst + SecureExecution> UniversalImpl for #self_ty {
                #(#items)*
            }
        })
    }
}

/// Default security implementation (permissive for development)
pub struct DefaultSecurity;

impl SecureExecution for DefaultSecurity {
    fn check_permission(&self, _operation: &AstOperation) -> bool {
        true // Allow all operations in development
    }
    
    fn sandbox_execute<F, T>(&self, f: F) -> Result<T, SecurityError>
    where F: FnOnce() -> T {
        Ok(f()) // No sandboxing in development
    }
}

/// Strict security implementation with ACL
pub struct StrictSecurity {
    allowed_operations: Vec<AstOperation>,
}

impl StrictSecurity {
    pub fn new(allowed_operations: Vec<AstOperation>) -> Self {
        Self { allowed_operations }
    }
}

impl SecureExecution for StrictSecurity {
    fn check_permission(&self, operation: &AstOperation) -> bool {
        self.allowed_operations.iter().any(|op| {
            std::mem::discriminant(op) == std::mem::discriminant(operation)
        })
    }
    
    fn sandbox_execute<F, T>(&self, f: F) -> Result<T, SecurityError>
    where F: FnOnce() -> T {
        // TODO: Implement actual sandboxing
        Ok(f())
    }
}
