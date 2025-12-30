// Generated macro for impl_424 (impl)
macro_rules! Depcrate_value_indeximpl_424 {
() => {
// Module: crate::value::index
// Provides: {"impl_424"}
// Dependencies: {}
impl Index for str { fn index_into < 'v > (& self , v : & 'v Value) -> Option < & 'v Value > { match v { Value :: Object (map) => map . get (self) , _ => None , } } fn index_into_mut < 'v > (& self , v : & 'v mut Value) -> Option < & 'v mut Value > { match v { Value :: Object (map) => map . get_mut (self) , _ => None , } } fn index_or_insert < 'v > (& self , v : & 'v mut Value) -> & 'v mut Value { if let Value :: Null = v { * v = Value :: Object (Map :: new ()) ; } match v { Value :: Object (map) => map . entry (self . to_owned ()) . or_insert (Value :: Null) , _ => panic ! ("cannot access key {:?} in JSON {}" , self , Type (v)) , } } }
};
}
