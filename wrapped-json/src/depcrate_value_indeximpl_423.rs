// Generated macro for impl_423 (impl)
macro_rules! Depcrate_value_indeximpl_423 {
() => {
// Module: crate::value::index
// Provides: {"impl_423"}
// Dependencies: {}
impl Index for usize { fn index_into < 'v > (& self , v : & 'v Value) -> Option < & 'v Value > { match v { Value :: Array (vec) => vec . get (* self) , _ => None , } } fn index_into_mut < 'v > (& self , v : & 'v mut Value) -> Option < & 'v mut Value > { match v { Value :: Array (vec) => vec . get_mut (* self) , _ => None , } } fn index_or_insert < 'v > (& self , v : & 'v mut Value) -> & 'v mut Value { match v { Value :: Array (vec) => { let len = vec . len () ; vec . get_mut (* self) . unwrap_or_else (| | { panic ! ("cannot access index {} of JSON array of length {}" , self , len) }) } _ => panic ! ("cannot access index {} of JSON {}" , self , Type (v)) , } } }
};
}
