// Generated macro for eyre (macro)
macro_rules! Depcrate_macroseyre {
() => {
// Module: crate::macros
// Provides: {"eyre"}
// Dependencies: {}
# [doc = " Construct an ad-hoc error from a string."] # [doc = ""] # [doc = " This evaluates to a `Report`. It can take either just a string, or a format"] # [doc = " string with arguments. It also can take any custom type which implements"] # [doc = " `Debug` and `Display`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # type V = ();"] # [doc = " #"] # [doc = " use eyre::{eyre, Result};"] # [doc = ""] # [doc = " fn lookup(key: &str) -> Result<V> {"] # [doc = "     if key.len() != 16 {"] # [doc = "         return Err(eyre!(\"key length must be 16 characters, got {:?}\", key));"] # [doc = "     }"] # [doc = ""] # [doc = "     // ..."] # [doc = "     # Ok(())"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! eyre { ($ msg : literal $ (,) ?) => ({ let error = $ crate :: private :: format_err ($ crate :: private :: format_args ! ($ msg)) ; error }) ; ($ err : expr $ (,) ?) => ({ use $ crate :: private :: kind ::*; let error = match $ err { error => (& error) . eyre_kind () . new (error) , } ; error }) ; ($ fmt : expr , $ ($ arg : tt) *) => { $ crate :: private :: new_adhoc ($ crate :: private :: format ! ($ fmt , $ ($ arg) *)) } ; }
};
}
