// Generated macro for contains (function)
macro_rules! Depcrate_mimecontains {
() => {
// Module: crate::mime
// Provides: {"contains"}
// Dependencies: {}
fn contains (parameters : & [(String , String)] , name : & str) -> bool { parameters . iter () . any (| (n , _) | n == name) }
};
}
