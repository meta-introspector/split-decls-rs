// Generated macro for impl_42 (impl)
macro_rules! Depcrate_program_mainimpl_42 {
() => {
// Module: crate::program::main
// Provides: {"impl_42"}
// Dependencies: {}
impl TryFrom < OsString > for Action { type Error = Error ; fn try_from (value : OsString) -> Result < Self , Self :: Error > { Ok (match value . to_str () { Some ("fill" | "get") => Action :: Get , Some ("approve" | "store") => Action :: Store , Some ("reject" | "erase") => Action :: Erase , _ => return Err (Error :: ActionInvalid { name : value }) , }) } }
};
}
