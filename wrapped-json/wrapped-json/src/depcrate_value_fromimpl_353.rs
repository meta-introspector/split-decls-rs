// Generated macro for impl_353 (impl)
macro_rules! Depcrate_value_fromimpl_353 {
() => {
// Module: crate::value::from
// Provides: {"impl_353"}
// Dependencies: {}
impl < T > From < Option < T > > for Value where T : Into < Value > , { fn from (opt : Option < T >) -> Self { match opt { None => Value :: Null , Some (value) => Into :: into (value) , } } }
};
}
