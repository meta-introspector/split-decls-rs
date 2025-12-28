macro_rules! bootstrap_cycle {
    () => {
        # [proc_macro] # [decl2 (fn , name = "bootstrap_cycle" , vis = "pub" , hash = "babd5455")] pub fn bootstrap_cycle (input : TokenStream) -> TokenStream { quine_relay :: bootstrap_cycle_impl (input) }
    };
}

bootstrap_cycle!();