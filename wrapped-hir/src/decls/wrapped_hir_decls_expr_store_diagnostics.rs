use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn expr_store_diagnostics<'db>(
    db: &'db dyn HirDatabase,
    acc: &mut Vec<AnyDiagnostic<'db>>,
    source_map: &ExpressionStoreSourceMap,
) {
    for diag in source_map.diagnostics() {
        acc.push(match diag {
            ExpressionStoreDiagnostics::InactiveCode { node, cfg, opts } => InactiveCode {
                node: *node,
                cfg: cfg.clone(),
                opts: opts.clone(),
            }
            .into(),
            ExpressionStoreDiagnostics::UnresolvedMacroCall { node, path } => UnresolvedMacroCall {
                macro_call: (*node).map(|ast_ptr| ast_ptr.into()),
                precise_location: None,
                path: path.clone(),
                is_bang: true,
            }
            .into(),
            ExpressionStoreDiagnostics::AwaitOutsideOfAsync { node, location } => {
                AwaitOutsideOfAsync {
                    node: *node,
                    location: location.clone(),
                }
                .into()
            }
            ExpressionStoreDiagnostics::UnreachableLabel { node, name } => UnreachableLabel {
                node: *node,
                name: name.clone(),
            }
            .into(),
            ExpressionStoreDiagnostics::UndeclaredLabel { node, name } => UndeclaredLabel {
                node: *node,
                name: name.clone(),
            }
            .into(),
        });
    }
    source_map
        .macro_calls()
        .for_each(|(_ast_id, call_id)| macro_call_diagnostics(db, call_id, acc));
}
