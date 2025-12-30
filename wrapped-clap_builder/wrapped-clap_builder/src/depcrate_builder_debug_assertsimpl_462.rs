// Generated macro for impl_462 (impl)
macro_rules! Depcrate_builder_debug_assertsimpl_462 {
() => {
// Module: crate::builder::debug_asserts
// Provides: {"impl_462"}
// Dependencies: {}
impl Ord for Flag < '_ > { fn cmp (& self , other : & Self) -> Ordering { match (self , other) { (Flag :: Command (s1 , _) , Flag :: Command (s2 , _)) | (Flag :: Arg (s1 , _) , Flag :: Arg (s2 , _)) | (Flag :: Command (s1 , _) , Flag :: Arg (s2 , _)) | (Flag :: Arg (s1 , _) , Flag :: Command (s2 , _)) => { if s1 == s2 { Ordering :: Equal } else { s1 . cmp (s2) } } } } }
};
}
