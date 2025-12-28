macro_rules! huggingface_event_impl {
    () => {
        # [decl (fn , name = "huggingface_event_impl" , vis = "pub" , hash = "05948575")] pub fn huggingface_event_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let model = input_str . value () ; quote ! { { let memory_item = format ! ("MemoryItem::HuggingFaceEvent {{ model: '{}', timestamp: {}, type: 'model_access' }}" , # model , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs ()) ; println ! ("cargo:warning=🤗 HuggingFace event: {}" , # model) ; memory_item } } . into () }
    };
}

huggingface_event_impl!()