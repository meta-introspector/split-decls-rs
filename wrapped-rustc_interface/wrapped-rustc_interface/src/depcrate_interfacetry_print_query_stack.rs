// Generated macro for try_print_query_stack (function)
macro_rules! Depcrate_interfacetry_print_query_stack {
() => {
// Module: crate::interface
// Provides: {"try_print_query_stack"}
// Dependencies: {}
pub fn try_print_query_stack (dcx : DiagCtxtHandle < '_ > , limit_frames : Option < usize > , file : Option < std :: fs :: File > ,) { eprintln ! ("query stack during panic:") ; let all_frames = ty :: tls :: with_context_opt (| icx | { if let Some (icx) = icx { ty :: print :: with_no_queries ! (print_query_stack (QueryCtxt :: new (icx . tcx) , icx . query , dcx , limit_frames , file ,)) } else { 0 } }) ; if let Some (limit_frames) = limit_frames && all_frames > limit_frames { eprintln ! ("... and {} other queries... use `env RUST_BACKTRACE=1` to see the full query stack" , all_frames - limit_frames) ; } else { eprintln ! ("end of query stack") ; } }
};
}
