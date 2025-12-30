// Generated macro for is_narrow_ptr (function)
macro_rules! Depcrate_volatile_compositesis_narrow_ptr {
() => {
// Module: crate::volatile_composites
// Provides: {"is_narrow_ptr"}
// Dependencies: {}
# [doc = " A thin raw pointer or reference."] fn is_narrow_ptr < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { match ty . kind () { ty :: RawPtr (inner , _) | ty :: Ref (_ , inner , _) => inner . has_trivial_sizedness (cx . tcx , ty :: SizedTraitKind :: Sized) , _ => false , } }
};
}
