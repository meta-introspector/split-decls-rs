// Generated macro for impl_79 (impl)
macro_rules! Depcrate_astimpl_79 {
() => {
// Module: crate::ast
// Provides: {"impl_79"}
// Dependencies: {}
impl BindingMode { pub const NONE : Self = Self (ByRef :: No , Mutability :: Not) ; pub const REF : Self = Self (ByRef :: Yes (Mutability :: Not) , Mutability :: Not) ; pub const MUT : Self = Self (ByRef :: No , Mutability :: Mut) ; pub const REF_MUT : Self = Self (ByRef :: Yes (Mutability :: Mut) , Mutability :: Not) ; pub const MUT_REF : Self = Self (ByRef :: Yes (Mutability :: Not) , Mutability :: Mut) ; pub const MUT_REF_MUT : Self = Self (ByRef :: Yes (Mutability :: Mut) , Mutability :: Mut) ; pub fn prefix_str (self) -> & 'static str { match self { Self :: NONE => "" , Self :: REF => "ref " , Self :: MUT => "mut " , Self :: REF_MUT => "ref mut " , Self :: MUT_REF => "mut ref " , Self :: MUT_REF_MUT => "mut ref mut " , } } }
};
}
