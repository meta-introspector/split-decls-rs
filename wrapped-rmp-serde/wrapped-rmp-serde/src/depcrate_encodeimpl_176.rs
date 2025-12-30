// Generated macro for impl_176 (impl)
macro_rules! Depcrate_encodeimpl_176 {
() => {
// Module: crate::encode
// Provides: {"impl_176"}
// Dependencies: {}
impl < 'a , W : Write + 'a > ExtFieldSerializer < 'a , W > { # [inline] fn new < C > (ser : & 'a mut Serializer < W , C >) -> Self { Self { wr : UnderlyingWrite :: get_mut (ser) , tag : None , finish : false , } } # [inline] const fn end (self) -> Result < () , Error > { if self . finish { Ok (()) } else { Err (Error :: InvalidDataModel ("expected i8 and bytes")) } } }
};
}
