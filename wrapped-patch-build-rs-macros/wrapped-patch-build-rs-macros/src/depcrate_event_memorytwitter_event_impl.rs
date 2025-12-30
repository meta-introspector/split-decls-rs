// Generated macro for twitter_event_impl (function)
macro_rules! Depcrate_event_memorytwitter_event_impl {
() => {
// Module: crate::event_memory
// Provides: {"twitter_event_impl"}
// Dependencies: {}
# [decl (fn , name = "twitter_event_impl" , vis = "pub" , hash = "9e24288b")] pub fn twitter_event_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let hashtag = input_str . value () ; quote ! { { let memory_item = format ! ("MemoryItem::TwitterEvent {{ hashtag: '{}', timestamp: {}, sentiment: 'neutral' }}" , # hashtag , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs ()) ; println ! ("cargo:warning=🐦 Twitter event: {}" , # hashtag) ; memory_item } } . into () }
};
}
