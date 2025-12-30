// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl < T > Option < T > { pub fn unwrap (self) -> T { match self { Some (val) => val , None => panic ! ("called `Option::unwrap()` on a `None` value") , } } }
};
}
