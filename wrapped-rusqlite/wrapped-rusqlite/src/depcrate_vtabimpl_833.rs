// Generated macro for impl_833 (impl)
macro_rules! Depcrate_vtabimpl_833 {
() => {
// Module: crate::vtab
// Provides: {"impl_833"}
// Dependencies: {}
# [cfg (feature = "modern_sqlite")] impl < 'a > Filters < 'a > { # [doc = " Find all elements on the right-hand side of an IN constraint"] pub fn in_values (& self , idx : usize) -> Result < InValues < '_ > > { let list = self . args [idx] ; Ok (InValues { list , phantom : PhantomData , first : true , }) } }
};
}
