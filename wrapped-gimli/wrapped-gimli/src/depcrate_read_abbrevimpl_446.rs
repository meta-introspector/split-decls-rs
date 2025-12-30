// Generated macro for impl_446 (impl)
macro_rules! Depcrate_read_abbrevimpl_446 {
() => {
// Module: crate::read::abbrev
// Provides: {"impl_446"}
// Dependencies: {}
impl FromIterator < AttributeSpecification > for Attributes { fn from_iter < I > (iter : I) -> Attributes where I : IntoIterator < Item = AttributeSpecification > , { let mut list = Attributes :: new () ; for item in iter { list . push (item) ; } list } }
};
}
