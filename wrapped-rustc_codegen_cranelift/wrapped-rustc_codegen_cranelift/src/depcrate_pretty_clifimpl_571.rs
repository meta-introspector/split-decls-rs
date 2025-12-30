// Generated macro for impl_571 (impl)
macro_rules! Depcrate_pretty_clifimpl_571 {
() => {
// Module: crate::pretty_clif
// Provides: {"impl_571"}
// Dependencies: {}
impl fmt :: Debug for FunctionCx < '_ , '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "{:?}" , self . instance . args) ? ; writeln ! (f , "{:?}" , self . local_map) ? ; let mut clif = String :: new () ; :: cranelift_codegen :: write :: decorate_function (& mut & self . clif_comments , & mut clif , & self . bcx . func ,) . unwrap () ; writeln ! (f , "\n{}" , clif) } }
};
}
