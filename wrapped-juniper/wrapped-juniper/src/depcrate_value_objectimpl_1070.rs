// Generated macro for impl_1070 (impl)
macro_rules! Depcrate_value_objectimpl_1070 {
() => {
// Module: crate::value::object
// Provides: {"impl_1070"}
// Dependencies: {}
impl < K , S > FromIterator < (K , Value < S >) > for Object < S > where K : AsRef < str > + Into < String > , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , Value < S >) > , { let iter = iter . into_iter () ; let mut ret = Self { key_value_list : IndexMap :: with_capacity (iter . size_hint () . 0) , } ; for (k , v) in iter { ret . add_field (k , v) ; } ret } }
};
}
