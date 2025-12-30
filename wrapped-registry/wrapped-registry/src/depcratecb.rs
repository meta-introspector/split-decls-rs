// Generated macro for cb (macro)
macro_rules! Depcratecb {
() => {
// Module: crate
// Provides: {"cb"}
// Dependencies: {}
macro_rules ! cb { ($ ($ marker_ty : ty :$ marker : ident ,) + # [experimental] $ ($ emarker_ty : ty :$ emarker : ident ,) +) => { # [test] fn no_marker_collisions () { use icu_provider :: prelude ::*; let mut map = std :: collections :: BTreeMap :: new () ; let mut failed = false ; for marker in [$ (<$ marker_ty >:: INFO ,) + $ (<$ emarker_ty >:: INFO ,) +] { if let Some (colliding_marker) = map . insert (marker . id . hashed () , marker) { println ! ("{:?} and {:?} collide at {:?}" , marker . id , colliding_marker . id , marker . id . hashed () ,) ; failed = true ; } } if failed { panic ! () ; } } } }
};
}
