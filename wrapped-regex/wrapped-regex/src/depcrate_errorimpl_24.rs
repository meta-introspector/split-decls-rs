// Generated macro for impl_24 (impl)
macro_rules! Depcrate_errorimpl_24 {
() => {
// Module: crate::error
// Provides: {"impl_24"}
// Dependencies: {}
impl core :: fmt :: Debug for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match * self { Error :: Syntax (ref err) => { let hr : String = core :: iter :: repeat ('~') . take (79) . collect () ; writeln ! (f , "Syntax(") ? ; writeln ! (f , "{hr}") ? ; writeln ! (f , "{err}") ? ; writeln ! (f , "{hr}") ? ; write ! (f , ")") ? ; Ok (()) } Error :: CompiledTooBig (limit) => { f . debug_tuple ("CompiledTooBig") . field (& limit) . finish () } } } }
};
}
