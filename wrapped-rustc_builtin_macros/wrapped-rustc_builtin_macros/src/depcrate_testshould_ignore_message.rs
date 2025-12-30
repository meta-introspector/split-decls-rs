// Generated macro for should_ignore_message (function)
macro_rules! Depcrate_testshould_ignore_message {
() => {
// Module: crate::test
// Provides: {"should_ignore_message"}
// Dependencies: {}
fn should_ignore_message (i : & ast :: Item) -> Option < Symbol > { match attr :: find_by_name (& i . attrs , sym :: ignore) { Some (attr) => { match attr . meta_item_list () { Some (_) => None , None => attr . value_str () , } } None => None , } }
};
}
