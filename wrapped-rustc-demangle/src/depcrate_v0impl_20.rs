// Generated macro for impl_20 (impl)
macro_rules! Depcrate_v0impl_20 {
() => {
// Module: crate::v0
// Provides: {"impl_20"}
// Dependencies: {}
impl < 's > fmt :: Display for Demangle < 's > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut printer = Printer { parser : Ok (Parser { sym : self . inner , next : 0 , depth : 0 , }) , out : Some (f) , bound_lifetime_depth : 0 , } ; printer . print_path (true) } }
};
}
