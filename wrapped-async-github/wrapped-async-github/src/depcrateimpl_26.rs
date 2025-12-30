// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl From < & PullRequest > for Row < '_ > { fn from (pr : & PullRequest) -> Self { let pr = pr . clone () ; Row :: new (vec ! [pr . id , pr . title , pr . url]) } }
};
}
