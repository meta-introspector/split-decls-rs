// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < A > core :: iter :: Extend < A > for Array where A : AsRef < JsValue > , { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = A > , { for value in iter { self . push (value . as_ref ()) ; } } }
};
}
