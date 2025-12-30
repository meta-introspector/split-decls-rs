// Generated macro for impl_132 (impl)
macro_rules! Depcrate_nfaimpl_132 {
() => {
// Module: crate::nfa
// Provides: {"impl_132"}
// Dependencies: {}
impl core :: fmt :: Debug for NFA { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { writeln ! (f , "NFA(") ? ; writeln ! (f , "pattern: {}" , self . pattern) ? ; for (sid , state) in self . states . iter () . enumerate () { writeln ! (f , "{sid:07?}: {state:?}") ? ; } writeln ! (f , ")") ? ; Ok (()) } }
};
}
