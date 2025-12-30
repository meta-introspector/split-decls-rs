// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl < T > Iterator for IdxRange < T > { type Item = Idx < T > ; fn next (& mut self) -> Option < Self :: Item > { self . range . next () . map (| raw | Idx :: from_raw (raw . into ())) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } fn count (self) -> usize where Self : Sized , { self . range . count () } fn last (self) -> Option < Self :: Item > where Self : Sized , { self . range . last () . map (| raw | Idx :: from_raw (raw . into ())) } fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . range . nth (n) . map (| raw | Idx :: from_raw (raw . into ())) } }
};
}
