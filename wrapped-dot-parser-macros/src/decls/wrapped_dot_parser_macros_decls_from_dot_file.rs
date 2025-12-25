use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Imports a DOT graph contained in a file.
///
/// Notice that the import happens *at compile time*. This
/// means the provided file *must* exist at compile time. This also means that modifying the graph
/// file without recompiling has no effect. This macro generates a graph
/// declaration that corresponds to the graph in the file. All the parsing happens *at compile
/// time*. If you want to import dynamically a graph (i.e. at compile time), use
/// `dot_parser::ast::Graph::from_file` instead.
///
/// This macro expects a single literal argument which is the path of the file to read.
///
/// This macro will fail if:
///  - the number of arguments is not exactly 1; or
///  - the file can not be read; or
///  - the content of the file is not a valid DOT graph.
///
/// For example, provided that the file `/tmp/graph.dot` contains:
/// ```ignore
/// digraph {
/// A -> B
/// }
/// ```
///
/// Using:
/// ```ignore
/// # use dot_parser_macros::from_dot;
/// let graph = from_dot!("/tmp/graph.dot");
/// ```
/// is roughtly equivalent to:
/// ```
/// # use dot_parser::ast::*;
/// # use dot_parser::ast::either::Either;
/// let graph = Graph::<(&'static str, &'static str)> {
///     strict: false,
///     is_digraph: true,
///     name: Option::None,
///     stmts: StmtList {
///         stmts: vec![
///             Stmt::EdgeStmt(EdgeStmt {
///                 from: Either::Left(NodeID {
///                     id: "A".to_string(),
///                     port: Option::None,
///                 }),
///                 next: EdgeRHS {
///                     to: Either::Left(NodeID {
///                         id: "B".to_string(),
///                         port: Option::None,
///                     }),
///                     next: Option::None,
///                 },
///                 attr: Option::None,
///             }),
///         ]
///     },
/// };
/// ```
#[proc_macro]
pub fn from_dot_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let tokens: Vec<_> = input.into_iter().collect();
    if tokens.len() != 1 {
        let output: proc_macro2::TokenStream = {
            quote! {
                compile_error!("Expect a single path argument")
            }
        };
        return proc_macro::TokenStream::from(output);
    }
    let token = tokens.into_iter().next().unwrap();
    let str_lit = match StringLit::try_from(token) {
        Err(e) => return e.to_compile_error(),
        Ok(lit) => lit,
    };
    let graph = match Graph::from_file(str_lit.value()) {
        Err(f) => {
            let msg = format!("{}", f);
            return quote! {
                compile_error!(# msg)
            }
            .into();
        }
        Ok(graph) => graph,
    };
    let output: proc_macro2::TokenStream = quote! {
        # graph
    };
    proc_macro::TokenStream::from(output)
}
