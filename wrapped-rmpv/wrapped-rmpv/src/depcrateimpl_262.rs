// Generated macro for impl_262 (impl)
macro_rules! Depcrateimpl_262 {
() => {
// Module: crate
// Provides: {"impl_262"}
// Dependencies: {}
impl Display for ValueRef < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { ValueRef :: Nil => write ! (f , "nil") , ValueRef :: Boolean (val) => Display :: fmt (& val , f) , ValueRef :: Integer (ref val) => Display :: fmt (& val , f) , ValueRef :: F32 (val) => Display :: fmt (& val , f) , ValueRef :: F64 (val) => Display :: fmt (& val , f) , ValueRef :: String (ref val) => Display :: fmt (& val , f) , ValueRef :: Binary (val) => Debug :: fmt (& & val , f) , ValueRef :: Array (ref vec) => { let res = vec . iter () . map (| val | format ! ("{val}")) . collect :: < Vec < String > > () . join (", ") ; write ! (f , "[{res}]") } ValueRef :: Map (ref vec) => { write ! (f , "{{") ? ; match vec . iter () . take (1) . next () { Some ((k , v)) => { write ! (f , "{k}: {v}") ? ; } None => { write ! (f , "") ? ; } } for (k , v) in vec . iter () . skip (1) { write ! (f , ", {k}: {v}") ? ; } write ! (f , "}}") } ValueRef :: Ext (ty , data) => { write ! (f , "[{ty}, {data:?}]") } } } }
};
}
