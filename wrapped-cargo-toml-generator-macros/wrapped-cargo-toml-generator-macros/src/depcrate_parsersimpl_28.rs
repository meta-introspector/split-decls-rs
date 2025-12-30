// Generated macro for impl_28 (impl)
macro_rules! Depcrate_parsersimpl_28 {
() => {
// Module: crate::parsers
// Provides: {"impl_28"}
// Dependencies: {}
impl KeyValue { pub fn get_ident_for_err (& self) -> & Ident { match self { KeyValue :: Simple (key , _) => key , KeyValue :: Block (key , _) => key , KeyValue :: List (key , _) => key , KeyValue :: InlineTable (key , _) => key , } } }
};
}
