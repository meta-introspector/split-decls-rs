macro_rules! archive_event {
    () => {
        # [proc_macro] # [decl2 (fn , name = "archive_event" , vis = "pub" , hash = "db2cb06b")] pub fn archive_event (input : TokenStream) -> TokenStream { event_memory :: archive_event_impl (input) }
    };
}

archive_event!()