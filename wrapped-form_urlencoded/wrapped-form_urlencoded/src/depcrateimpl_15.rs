// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl Iterator for ParseIntoOwned < '_ > { type Item = (String , String) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (k , v) | (k . into_owned () , v . into_owned ())) } }
};
}
