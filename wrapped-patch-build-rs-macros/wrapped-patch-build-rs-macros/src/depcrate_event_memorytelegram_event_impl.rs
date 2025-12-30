// Generated macro for telegram_event_impl (function)
macro_rules! Depcrate_event_memorytelegram_event_impl {
() => {
// Module: crate::event_memory
// Provides: {"telegram_event_impl"}
// Dependencies: {}
# [decl (fn , name = "telegram_event_impl" , vis = "pub" , hash = "c6e98ffc")] pub fn telegram_event_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let channel = input_str . value () ; quote ! { { let memory_item = format ! ("MemoryItem::TelegramEvent {{ channel: '{}', timestamp: {}, activity: 'message' }}" , # channel , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs ()) ; println ! ("cargo:warning=✈️ Telegram event: {}" , # channel) ; memory_item } } . into () }
};
}
