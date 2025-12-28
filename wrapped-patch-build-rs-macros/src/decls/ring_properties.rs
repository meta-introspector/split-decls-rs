macro_rules! ring_properties {
    () => {
        # [proc_macro] # [decl2 (fn , name = "ring_properties" , vis = "pub" , hash = "2c553716")] pub fn ring_properties (input : TokenStream) -> TokenStream { rustc_ring :: ring_properties_impl (input) }
    };
}

ring_properties!()