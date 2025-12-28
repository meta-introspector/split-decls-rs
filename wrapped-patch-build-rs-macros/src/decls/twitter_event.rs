macro_rules! twitter_event {
    () => {
        # [proc_macro] # [decl2 (fn , name = "twitter_event" , vis = "pub" , hash = "1e0eff90")] pub fn twitter_event (input : TokenStream) -> TokenStream { event_memory :: twitter_event_impl (input) }
    };
}

twitter_event!()