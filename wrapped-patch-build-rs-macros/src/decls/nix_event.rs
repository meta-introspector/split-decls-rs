macro_rules! nix_event {
    () => {
        # [proc_macro] # [decl2 (fn , name = "nix_event" , vis = "pub" , hash = "a9209106")] pub fn nix_event (input : TokenStream) -> TokenStream { event_memory :: nix_event_impl (input) }
    };
}

nix_event!();