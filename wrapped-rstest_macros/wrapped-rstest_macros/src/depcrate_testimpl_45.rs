// Generated macro for impl_45 (impl)
macro_rules! Depcrate_testimpl_45 {
() => {
// Module: crate::test
// Provides: {"impl_45"}
// Dependencies: {}
impl Attribute { pub fn attr < S : AsRef < str > > (s : S) -> Self { Attribute :: Attr (ident (s)) } pub fn tagged < SI : AsRef < str > , SA : AsRef < str > > (tag : SI , attrs : Vec < SA >) -> Self { Attribute :: Tagged (ident (tag) , attrs . into_iter () . map (pat) . collect ()) } pub fn typed < S : AsRef < str > , T : AsRef < str > > (tag : S , inner : T) -> Self { Attribute :: Type (ident (tag) , parse_str (inner . as_ref ()) . unwrap ()) } }
};
}
