macro_rules! github_event {
    () => {
        # [proc_macro] # [decl2 (fn , name = "github_event" , vis = "pub" , hash = "405bb8fa")] pub fn github_event (input : TokenStream) -> TokenStream { event_memory :: github_event_impl (input) }
    };
}

github_event!();