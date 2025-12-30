// Generated macro for impl_62 (impl)
macro_rules! Depcrate_serde_implimpl_62 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for GlobSetVisitor { type Value = GlobSet ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter ,) -> std :: fmt :: Result { formatter . write_str ("an array of glob patterns") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut builder = GlobSetBuilder :: new () ; while let Some (glob) = seq . next_element () ? { builder . add (glob) ; } builder . build () . map_err (serde :: de :: Error :: custom) } }
};
}
