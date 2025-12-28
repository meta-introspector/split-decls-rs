macro_rules! dependency_graph {
    () => {
        # [proc_macro] # [decl2 (fn , name = "dependency_graph" , vis = "pub" , hash = "55226dc5")] pub fn dependency_graph (input : TokenStream) -> TokenStream { rustc_ring :: dependency_graph_impl (input) }
    };
}

dependency_graph!();