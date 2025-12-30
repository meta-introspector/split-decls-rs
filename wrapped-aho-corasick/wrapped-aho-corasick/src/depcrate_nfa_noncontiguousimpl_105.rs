// Generated macro for impl_105 (impl)
macro_rules! Depcrate_nfa_noncontiguousimpl_105 {
() => {
// Module: crate::nfa::noncontiguous
// Provides: {"impl_105"}
// Dependencies: {}
impl core :: fmt :: Debug for Transition { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Transition(byte: {:X?}, next: {:?}, link: {:?})" , self . byte , self . next () . as_usize () , self . link () . as_usize ()) } }
};
}
