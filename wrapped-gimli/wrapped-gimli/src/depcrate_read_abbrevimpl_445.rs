// Generated macro for impl_445 (impl)
macro_rules! Depcrate_read_abbrevimpl_445 {
() => {
// Module: crate::read::abbrev
// Provides: {"impl_445"}
// Dependencies: {}
impl Deref for Attributes { type Target = [AttributeSpecification] ; fn deref (& self) -> & [AttributeSpecification] { match self { Attributes :: Inline { buf , len } => & buf [.. * len] , Attributes :: Heap (list) => list , } } }
};
}
