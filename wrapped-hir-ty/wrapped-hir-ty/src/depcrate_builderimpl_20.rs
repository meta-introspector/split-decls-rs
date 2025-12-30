// Generated macro for impl_20 (impl)
macro_rules! Depcrate_builderimpl_20 {
() => {
// Module: crate::builder
// Provides: {"impl_20"}
// Dependencies: {}
impl TyBuilder < Tuple > { pub fn tuple (size : usize) -> TyBuilder < Tuple > { TyBuilder :: new (Tuple (size) , std :: iter :: repeat_n (ParamKind :: Type , size) . collect () , None) } pub fn build (self) -> Ty { let (Tuple (size) , subst) = self . build_internal () ; TyKind :: Tuple (size , subst) . intern (Interner) } pub fn tuple_with < I > (elements : I) -> Ty where I : IntoIterator < Item = Ty > , < I as IntoIterator > :: IntoIter : ExactSizeIterator , { let elements = elements . into_iter () ; let len = elements . len () ; let mut b = TyBuilder :: new (Tuple (len) , std :: iter :: repeat_n (ParamKind :: Type , len) . collect () , None) ; for e in elements { b = b . push (e) ; } b . build () } }
};
}
