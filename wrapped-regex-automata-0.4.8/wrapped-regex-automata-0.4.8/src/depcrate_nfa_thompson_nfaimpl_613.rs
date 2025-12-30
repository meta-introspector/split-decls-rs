// Generated macro for impl_613 (impl)
macro_rules! Depcrate_nfa_thompson_nfaimpl_613 {
() => {
// Module: crate::nfa::thompson::nfa
// Provides: {"impl_613"}
// Dependencies: {}
impl fmt :: Debug for Transition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use crate :: util :: escape :: DebugByte ; let Transition { start , end , next } = * self ; if self . start == self . end { write ! (f , "{:?} => {:?}" , DebugByte (start) , next . as_usize ()) } else { write ! (f , "{:?}-{:?} => {:?}" , DebugByte (start) , DebugByte (end) , next . as_usize () ,) } } }
};
}
