// Generated macro for impl_38 (impl)
macro_rules! Depcrate_descriptorimpl_38 {
() => {
// Module: crate::descriptor
// Provides: {"impl_38"}
// Dependencies: {}
impl Closure { fn decode (data : & mut & [u32]) -> Closure { let dtor_idx = get (data) ; let mutable = match get (data) { 0 => false , 1 => true , other => panic ! ("expected bool value, got {other}") , } ; assert_eq ! (get (data) , FUNCTION) ; Closure { dtor_idx , mutable , function : Function :: decode (data) , } } }
};
}
