// Generated macro for impl_938 (impl)
macro_rules! Depcrate_util_shapeimpl_938 {
() => {
// Module: crate::util::shape
// Provides: {"impl_938"}
// Dependencies: {}
impl fmt :: Display for ShapeSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let shapes = self . to_vec () ; match shapes . len () { 0 => write ! (f , "nothing") , 1 => write ! (f , "{}" , shapes [0]) , 2 => write ! (f , "{} or {}" , shapes [0] , shapes [1]) , 3 => write ! (f , "{}, {}, or {}" , shapes [0] , shapes [1] , shapes [2]) , _ => unreachable ! () , } } }
};
}
