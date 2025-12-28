macro_rules! archive_event_impl {
    () => {
        # [decl (fn , name = "archive_event_impl" , vis = "pub" , hash = "98d37693")] pub fn archive_event_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let url = input_str . value () ; quote ! { { let memory_item = format ! ("MemoryItem::ArchiveEvent {{ url: '{}', timestamp: {}, status: 'archived' }}" , # url , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs ()) ; println ! ("cargo:warning=📚 Archive event: {}" , # url) ; memory_item } } . into () }
    };
}

archive_event_impl!()