// Generated macro for hermit_var (macro)
macro_rules! Depcrate_macroshermit_var {
() => {
// Module: crate::macros
// Provides: {"hermit_var"}
// Dependencies: {}
# [doc = " Returns the value of the specified environment variable."] # [doc = ""] # [doc = " The value is fetched from the current runtime environment and, if not"] # [doc = " present, falls back to the same environment variable set at compile time"] # [doc = " (might not be present as well)."] # [allow (unused_macros)] macro_rules ! hermit_var { ($ name : expr) => { { use alloc :: borrow :: Cow ; match crate :: env :: var ($ name) { Some (val) => Some (Cow :: from (val)) , None => option_env ! ($ name) . map (Cow :: Borrowed) , } } } ; }
};
}
