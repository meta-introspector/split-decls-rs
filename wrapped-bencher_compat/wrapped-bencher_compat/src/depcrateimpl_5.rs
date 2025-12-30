// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl < 'a , 'b > Bencher < 'a , 'b > { # [doc = " Callback for benchmark functions to run to perform the benchmark"] pub fn iter < T , F > (& mut self , inner : F) where F : FnMut () -> T { self . bencher . iter (inner) ; } }
};
}
