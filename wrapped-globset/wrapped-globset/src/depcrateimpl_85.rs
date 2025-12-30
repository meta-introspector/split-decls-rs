// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
impl GlobSetMatchStrategy { fn is_match (& self , candidate : & Candidate < '_ >) -> bool { use self :: GlobSetMatchStrategy :: * ; match * self { Literal (ref s) => s . is_match (candidate) , BasenameLiteral (ref s) => s . is_match (candidate) , Extension (ref s) => s . is_match (candidate) , Prefix (ref s) => s . is_match (candidate) , Suffix (ref s) => s . is_match (candidate) , RequiredExtension (ref s) => s . is_match (candidate) , Regex (ref s) => s . is_match (candidate) , } } fn matches_into (& self , candidate : & Candidate < '_ > , matches : & mut Vec < usize > ,) { use self :: GlobSetMatchStrategy :: * ; match * self { Literal (ref s) => s . matches_into (candidate , matches) , BasenameLiteral (ref s) => s . matches_into (candidate , matches) , Extension (ref s) => s . matches_into (candidate , matches) , Prefix (ref s) => s . matches_into (candidate , matches) , Suffix (ref s) => s . matches_into (candidate , matches) , RequiredExtension (ref s) => s . matches_into (candidate , matches) , Regex (ref s) => s . matches_into (candidate , matches) , } } }
};
}
