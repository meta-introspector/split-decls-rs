// Generated macro for impl_43 (impl)
macro_rules! Depcrate_inputimpl_43 {
() => {
// Module: crate::input
// Provides: {"impl_43"}
// Dependencies: {}
impl CrateName { # [doc = " Creates a crate name, checking for dashes in the string provided."] # [doc = " Dashes are not allowed in the crate names,"] # [doc = " hence the input string is returned as `Err` for those cases."] pub fn new (name : & str) -> Result < CrateName , & str > { if name . contains ('-') { Err (name) } else { Ok (Self (Symbol :: intern (name))) } } # [doc = " Creates a crate name, unconditionally replacing the dashes with underscores."] pub fn normalize_dashes (name : & str) -> CrateName { Self (Symbol :: intern (& name . replace ('-' , "_"))) } pub fn symbol (& self) -> & Symbol { & self . 0 } }
};
}
