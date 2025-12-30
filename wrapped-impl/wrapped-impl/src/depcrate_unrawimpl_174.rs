// Generated macro for impl_174 (impl)
macro_rules! Depcrate_unrawimpl_174 {
() => {
// Module: crate::unraw
// Provides: {"impl_174"}
// Dependencies: {}
impl Hash for MemberUnraw { fn hash < H : Hasher > (& self , hasher : & mut H) { match self { MemberUnraw :: Named (ident) => ident . 0 . unraw () . hash (hasher) , MemberUnraw :: Unnamed (index) => index . hash (hasher) , } } }
};
}
