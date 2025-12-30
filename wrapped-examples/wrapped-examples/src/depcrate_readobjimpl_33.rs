// Generated macro for impl_33 (impl)
macro_rules! Depcrate_readobjimpl_33 {
() => {
// Module: crate::readobj
// Provides: {"impl_33"}
// Dependencies: {}
impl < T , E : fmt :: Display > PrintErr < T > for Result < T , E > { fn print_err (self , p : & mut Printer < '_ >) -> Option < T > { match self { Ok (val) => Some (val) , Err (err) => { writeln ! (p . e , "Error: {}" , err) . unwrap () ; None } } } }
};
}
