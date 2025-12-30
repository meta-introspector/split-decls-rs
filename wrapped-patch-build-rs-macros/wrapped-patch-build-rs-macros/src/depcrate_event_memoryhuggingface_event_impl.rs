// Generated macro for huggingface_event_impl (function)
macro_rules! Depcrate_event_memoryhuggingface_event_impl {
() => {
// Module: crate::event_memory
// Provides: {"huggingface_event_impl"}
// Dependencies: {}
# [decl (fn , name = "huggingface_event_impl" , vis = "pub" , hash = "05948575")] pub fn huggingface_event_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let model = input_str . value () ; quote ! { { let memory_item = format ! ("MemoryItem::HuggingFaceEvent {{ model: '{}', timestamp: {}, type: 'model_access' }}" , # model , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs ()) ; println ! ("cargo:warning=🤗 HuggingFace event: {}" , # model) ; memory_item } } . into () }
};
}
