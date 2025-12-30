// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < T : Internable + ? Sized > Interned < T > { # [inline] pub fn new_generic < U > (obj : U) -> Self where U : Borrow < T > , Arc < T > : From < U > , { let storage = T :: storage () . get () ; let new_arc = Arc :: from (obj) ; let entry = storage . entry (new_arc . clone ()) . or_insert (()) ; Self { arc : entry . key () . clone () } } }
};
}
