use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn emit_def_diagnostic_<'db>(
    db: &'db dyn HirDatabase,
    acc: &mut Vec<AnyDiagnostic<'db>>,
    diag: &DefDiagnosticKind,
    edition: Edition,
) {
    match diag {
        DefDiagnosticKind::UnresolvedModule {
            ast: declaration,
            candidates,
        } => {
            let decl = declaration.to_ptr(db);
            acc.push(
                UnresolvedModule {
                    decl: InFile::new(declaration.file_id, decl),
                    candidates: candidates.clone(),
                }
                .into(),
            )
        }
        DefDiagnosticKind::UnresolvedExternCrate { ast } => {
            let item = ast.to_ptr(db);
            acc.push(
                UnresolvedExternCrate {
                    decl: InFile::new(ast.file_id, item),
                }
                .into(),
            );
        }
        DefDiagnosticKind::MacroError { ast, path, err } => {
            let item = ast.to_ptr(db);
            let RenderedExpandError {
                message,
                error,
                kind,
            } = err.render_to_string(db);
            acc.push(
                MacroError {
                    node: InFile::new(ast.file_id, item.syntax_node_ptr()),
                    precise_location: None,
                    message: format!("{}: {message}", path.display(db, edition)),
                    error,
                    kind,
                }
                .into(),
            )
        }
        DefDiagnosticKind::UnresolvedImport { id, index } => {
            let file_id = id.file_id;
            let use_tree = hir_def::src::use_tree_to_ast(db, *id, *index);
            acc.push(
                UnresolvedImport {
                    decl: InFile::new(file_id, AstPtr::new(&use_tree)),
                }
                .into(),
            );
        }
        DefDiagnosticKind::UnconfiguredCode { ast_id, cfg, opts } => {
            let ast_id_map = db.ast_id_map(ast_id.file_id);
            let ptr = ast_id_map.get_erased(ast_id.value);
            acc.push(
                InactiveCode {
                    node: InFile::new(ast_id.file_id, ptr),
                    cfg: cfg.clone(),
                    opts: opts.clone(),
                }
                .into(),
            );
        }
        DefDiagnosticKind::UnresolvedMacroCall { ast, path } => {
            let (node, precise_location) = precise_macro_call_location(ast, db);
            acc.push(
                UnresolvedMacroCall {
                    macro_call: node,
                    precise_location,
                    path: path.clone(),
                    is_bang: matches!(ast, MacroCallKind::FnLike { .. }),
                }
                .into(),
            );
        }
        DefDiagnosticKind::UnimplementedBuiltinMacro { ast } => {
            let node = ast.to_node(db);
            let name = node
                .name()
                .expect("unimplemented builtin macro with no name");
            acc.push(
                UnimplementedBuiltinMacro {
                    node: ast.with_value(SyntaxNodePtr::from(AstPtr::new(&name))),
                }
                .into(),
            );
        }
        DefDiagnosticKind::InvalidDeriveTarget { ast, id } => {
            let node = ast.to_node(db);
            let derive = node.attrs().nth(*id);
            match derive {
                Some(derive) => {
                    acc.push(
                        InvalidDeriveTarget {
                            node: ast.with_value(SyntaxNodePtr::from(AstPtr::new(&derive))),
                        }
                        .into(),
                    );
                }
                None => {
                    stdx::never!("derive diagnostic on item without derive attribute")
                }
            }
        }
        DefDiagnosticKind::MalformedDerive { ast, id } => {
            let node = ast.to_node(db);
            let derive = node.attrs().nth(*id);
            match derive {
                Some(derive) => {
                    acc.push(
                        MalformedDerive {
                            node: ast.with_value(SyntaxNodePtr::from(AstPtr::new(&derive))),
                        }
                        .into(),
                    );
                }
                None => {
                    stdx::never!("derive diagnostic on item without derive attribute")
                }
            }
        }
        DefDiagnosticKind::MacroDefError { ast, message } => {
            let node = ast.to_node(db);
            acc.push(
                MacroDefError {
                    node: InFile::new(ast.file_id, AstPtr::new(&node)),
                    name: node.name().map(|it| it.syntax().text_range()),
                    message: message.clone(),
                }
                .into(),
            );
        }
    }
}
