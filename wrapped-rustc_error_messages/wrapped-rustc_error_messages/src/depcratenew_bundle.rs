// Generated macro for new_bundle (function)
macro_rules! Depcratenew_bundle {
() => {
// Module: crate
// Provides: {"new_bundle"}
// Dependencies: {}
fn new_bundle (locales : Vec < LanguageIdentifier >) -> FluentBundle { IntoDynSyncSend (fluent_bundle :: bundle :: FluentBundle :: new_concurrent (locales)) }
};
}
