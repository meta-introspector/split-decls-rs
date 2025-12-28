macro_rules! memory_select {
    () => {
        # [proc_macro] # [decl2 (fn , name = "memory_select" , vis = "pub" , hash = "847b27a8")] pub fn memory_select (input : TokenStream) -> TokenStream { graph_partition :: memory_select_impl (input) }
    };
}

memory_select!();