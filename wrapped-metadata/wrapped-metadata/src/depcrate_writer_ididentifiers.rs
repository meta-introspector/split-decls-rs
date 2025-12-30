// Generated macro for identifiers (macro)
macro_rules! Depcrate_writer_ididentifiers {
() => {
// Module: crate::writer::id
// Provides: {"identifiers"}
// Dependencies: {}
macro_rules ! identifiers { ($ ($ name : ident) +) => { $ (# [derive (Default , Copy , Clone , Hash , PartialEq , Eq , Ord , PartialOrd , Debug)] pub struct $ name (pub (crate) u32) ;) * } ; }
};
}
