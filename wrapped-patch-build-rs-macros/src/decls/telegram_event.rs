macro_rules! telegram_event {
    () => {
        # [proc_macro] # [decl2 (fn , name = "telegram_event" , vis = "pub" , hash = "e7939c6a")] pub fn telegram_event (input : TokenStream) -> TokenStream { event_memory :: telegram_event_impl (input) }
    };
}

telegram_event!()