// Generated macro for tests (module)
macro_rules! Depcrate_texttests {
() => {
// Module: crate::text
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: vec ; use ratatui_core :: text :: Text ; # [test] fn text () { let text = text ! ["hello" , "world"] ; assert_eq ! (text , Text :: from (vec ! ["hello" . into () , "world" . into ()])) ; let text = text ! [crate :: line ! ("hello") , crate :: span ! ["world"]] ; assert_eq ! (text , Text :: from (vec ! ["hello" . into () , "world" . into ()])) ; let text = text ! ["hello" ; 2] ; assert_eq ! (text , Text :: from (vec ! ["hello" . into () , "hello" . into ()])) ; } }
};
}
