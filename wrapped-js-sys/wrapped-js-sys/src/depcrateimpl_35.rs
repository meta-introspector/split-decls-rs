// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl < A > core :: iter :: FromIterator < A > for Array where A : AsRef < JsValue > , { fn from_iter < T > (iter : T) -> Array where T : IntoIterator < Item = A > , { let mut out = Array :: new () ; out . extend (iter) ; out } }
};
}
