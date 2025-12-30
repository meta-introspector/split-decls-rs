// Generated macro for impl_226 (impl)
macro_rules! Depcrate_valueimpl_226 {
() => {
// Module: crate::value
// Provides: {"impl_226"}
// Dependencies: {}
impl Ord for Value { fn cmp (& self , other : & Value) -> Ordering { use self :: Value :: * ; if self . major_type () != other . major_type () { return self . major_type () . cmp (& other . major_type ()) ; } match (self , other) { (Integer (a) , Integer (b)) => a . abs () . cmp (& b . abs ()) , (Bytes (a) , Bytes (b)) if a . len () != b . len () => a . len () . cmp (& b . len ()) , (Text (a) , Text (b)) if a . len () != b . len () => a . len () . cmp (& b . len ()) , (Array (a) , Array (b)) if a . len () != b . len () => a . len () . cmp (& b . len ()) , (Map (a) , Map (b)) if a . len () != b . len () => a . len () . cmp (& b . len ()) , (Bytes (a) , Bytes (b)) => a . cmp (b) , (Text (a) , Text (b)) => a . cmp (b) , (a , b) => { let a = crate :: to_vec (a) . expect ("self is serializable") ; let b = crate :: to_vec (b) . expect ("other is serializable") ; a . cmp (& b) } } } }
};
}
