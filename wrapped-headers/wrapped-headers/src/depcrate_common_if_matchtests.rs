// Generated macro for tests (module)
macro_rules! Depcrate_common_if_matchtests {
() => {
// Module: crate::common::if_match
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn is_any () { assert ! (IfMatch :: any () . is_any ()) ; assert ! (! IfMatch :: from (ETag :: from_static ("\"yolo\"")) . is_any ()) ; } # [test] fn precondition_fails () { let if_match = IfMatch :: from (ETag :: from_static ("\"foo\"")) ; let bar = ETag :: from_static ("\"bar\"") ; let weak_foo = ETag :: from_static ("W/\"foo\"") ; assert ! (! if_match . precondition_passes (& bar)) ; assert ! (! if_match . precondition_passes (& weak_foo)) ; } # [test] fn precondition_passes () { let foo = ETag :: from_static ("\"foo\"") ; let if_match = IfMatch :: from (foo . clone ()) ; assert ! (if_match . precondition_passes (& foo)) ; } # [test] fn precondition_any () { let foo = ETag :: from_static ("\"foo\"") ; let if_match = IfMatch :: any () ; assert ! (if_match . precondition_passes (& foo)) ; } }
};
}
