// Generated macro for impl_170 (impl)
macro_rules! Depcrate_unrawimpl_170 {
() => {
// Module: crate::unraw
// Provides: {"impl_170"}
// Dependencies: {}
impl Display for MemberUnraw { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self { MemberUnraw :: Named (this) => Display :: fmt (this , formatter) , MemberUnraw :: Unnamed (this) => Display :: fmt (& this . index , formatter) , } } }
};
}
