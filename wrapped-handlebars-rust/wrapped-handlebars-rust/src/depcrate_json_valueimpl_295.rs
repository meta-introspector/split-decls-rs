// Generated macro for impl_295 (impl)
macro_rules! Depcrate_json_valueimpl_295 {
() => {
// Module: crate::json::value
// Provides: {"impl_295"}
// Dependencies: {}
impl JsonTruthy for Json { fn is_truthy (& self , include_zero : bool) -> bool { match * self { Json :: Bool (ref i) => * i , Json :: Number (ref n) => { if include_zero { n . as_f64 () . is_some_and (| f | ! f . is_nan ()) } else { n . as_f64 () . is_some_and (f64 :: is_normal) } } Json :: Null => false , Json :: String (ref i) => ! i . is_empty () , Json :: Array (ref i) => ! i . is_empty () , Json :: Object (ref i) => ! i . is_empty () , } } }
};
}
