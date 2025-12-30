// Generated macro for impl_175 (impl)
macro_rules! Depcrate_encodeimpl_175 {
() => {
// Module: crate::encode
// Provides: {"impl_175"}
// Dependencies: {}
impl < 'a , W : Write + 'a > ExtSerializer < 'a , W > { # [inline] fn new < C > (ser : & 'a mut Serializer < W , C >) -> Self { Self { fields_se : ExtFieldSerializer :: new (ser) , tuple_received : false , } } # [inline] const fn end (self) -> Result < () , Error > { if self . tuple_received { self . fields_se . end () } else { Err (Error :: InvalidDataModel ("expected tuple")) } } }
};
}
