// Generated macro for impl_234 (impl)
macro_rules! Depcrateimpl_234 {
() => {
// Module: crate
// Provides: {"impl_234"}
// Dependencies: {}
impl Display for Value { # [cold] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { Self :: Nil => f . write_str ("nil") , Self :: Boolean (val) => Display :: fmt (& val , f) , Self :: Integer (ref val) => Display :: fmt (& val , f) , Self :: F32 (val) => Display :: fmt (& val , f) , Self :: F64 (val) => Display :: fmt (& val , f) , Self :: String (ref val) => Display :: fmt (& val , f) , Self :: Binary (ref val) => Debug :: fmt (& val , f) , Self :: Array (ref vec) => { let res = vec . iter () . map (| val | format ! ("{val}")) . collect :: < Vec < String > > () . join (", ") ; write ! (f , "[{res}]") } Self :: Map (ref vec) => { write ! (f , "{{") ? ; match vec . iter () . take (1) . next () { Some ((k , v)) => { write ! (f , "{k}: {v}") ? ; } None => { write ! (f , "") ? ; } } for (k , v) in vec . iter () . skip (1) { write ! (f , ", {k}: {v}") ? ; } write ! (f , "}}") } Self :: Ext (ty , ref data) => { write ! (f , "[{ty}, {data:?}]") } } } }
};
}
