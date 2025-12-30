// Generated macro for tests (module)
macro_rules! Depcrate_availabilitytests {
() => {
// Module: crate::availability
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; fn single (aval : & mut Availability , idx : usize) { aval . set_available (idx , true) ; assert ! (aval . available ()) ; aval . set_available (idx , true) ; aval . set_available (idx , false) ; assert ! (! aval . available ()) ; aval . set_available (idx , false) ; assert ! (! aval . available ()) ; } fn multi (aval : & mut Availability , mut idx : Vec < usize >) { idx . iter () . for_each (| idx | aval . set_available (* idx , true)) ; assert ! (aval . available ()) ; while let Some (idx) = idx . pop () { assert ! (aval . available ()) ; aval . set_available (idx , false) ; } assert ! (! aval . available ()) ; } # [test] fn availability () { let mut aval = Availability :: default () ; single (& mut aval , 1) ; single (& mut aval , 128) ; single (& mut aval , 256) ; single (& mut aval , 511) ; let idx = (0 .. 511) . filter (| i | i % 3 == 0 && i % 5 == 0) . collect () ; multi (& mut aval , idx) ; multi (& mut aval , (0 .. 511) . collect ()) } # [test] # [should_panic] fn overflow () { let mut aval = Availability :: default () ; single (& mut aval , 512) ; } # [test] fn pin_point () { let mut aval = Availability :: default () ; aval . set_available (438 , true) ; aval . set_available (479 , true) ; assert_eq ! (aval . 0 [3] , 1 << (438 - 384) | 1 << (479 - 384)) ; } }
};
}
