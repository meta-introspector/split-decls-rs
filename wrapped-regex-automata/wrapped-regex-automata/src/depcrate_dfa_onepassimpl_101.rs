// Generated macro for impl_101 (impl)
macro_rules! Depcrate_dfa_onepassimpl_101 {
() => {
// Module: crate::dfa::onepass
// Provides: {"impl_101"}
// Dependencies: {}
impl core :: fmt :: Debug for Epsilons { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut wrote = false ; if ! self . slots () . is_empty () { write ! (f , "{:?}" , self . slots ()) ? ; wrote = true ; } if ! self . looks () . is_empty () { if wrote { write ! (f , "/") ? ; } write ! (f , "{:?}" , self . looks ()) ? ; wrote = true ; } if ! wrote { write ! (f , "N/A") ? ; } Ok (()) } }
};
}
