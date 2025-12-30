// Generated macro for implfrom (macro)
macro_rules! Depcrate_value_integerimplfrom {
() => {
// Module: crate::value::integer
// Provides: {"implfrom"}
// Dependencies: {}
macro_rules ! implfrom { ($ ($ (# [$ ($ attr : meta) +]) ? $ t : ident) +) => { $ ($ (# [$ ($ attr) +]) ? impl From <$ t > for Integer { # [inline] fn from (value : $ t) -> Self { Self (value as _) } } impl TryFrom < Integer > for $ t { type Error = core :: num :: TryFromIntError ; # [inline] fn try_from (value : Integer) -> Result < Self , Self :: Error > { $ t :: try_from (value . 0) } }) + } ; }
};
}
