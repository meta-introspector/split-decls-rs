// Generated macro for assert_done (function)
macro_rules! Depcrate_all_supportassert_done {
() => {
// Module: crate::all::support
// Provides: {"assert_done"}
// Dependencies: {}
pub fn assert_done < T , F > (mut f : F , result : Result < T :: Item , T :: Error >) where T : Future , T :: Item : Eq + fmt :: Debug , T :: Error : Eq + fmt :: Debug , F : FnMut () -> T , { let mut a = f () ; assert_eq ! (& a . poll (& mut Task :: new ()) . unwrap () , & result) ; drop (a) ; }
};
}
