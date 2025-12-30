// Generated macro for ref_name (function)
macro_rules! Depcrate_buildref_name {
() => {
// Module: crate::build
// Provides: {"ref_name"}
// Dependencies: {}
fn ref_name (input : & ForeignType) -> Ident { Ident :: new (& format ! ("{}Ref" , input . name) , input . name . span ()) }
};
}
