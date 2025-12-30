// Generated macro for find_member (function)
macro_rules! Depcrate_objdumpfind_member {
() => {
// Module: crate::objdump
// Provides: {"find_member"}
// Dependencies: {}
fn find_member (member_names : & mut [(String , bool)] , name : & [u8]) -> bool { if member_names . is_empty () { return true ; } match member_names . iter () . position (| x | x . 0 . as_bytes () == name) { Some (i) => { member_names [i] . 1 = true ; true } None => false , } }
};
}
