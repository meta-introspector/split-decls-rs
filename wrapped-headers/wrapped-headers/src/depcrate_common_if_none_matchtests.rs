// Generated macro for tests (module)
macro_rules! Depcrate_common_if_none_matchtests {
() => {
// Module: crate::common::if_none_match
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn precondition_fails () { let foo = ETag :: from_static ("\"foo\"") ; let weak_foo = ETag :: from_static ("W/\"foo\"") ; let if_none = IfNoneMatch :: from (foo . clone ()) ; assert ! (! if_none . precondition_passes (& foo)) ; assert ! (! if_none . precondition_passes (& weak_foo)) ; } # [test] fn precondition_passes () { let if_none = IfNoneMatch :: from (ETag :: from_static ("\"foo\"")) ; let bar = ETag :: from_static ("\"bar\"") ; let weak_bar = ETag :: from_static ("W/\"bar\"") ; assert ! (if_none . precondition_passes (& bar)) ; assert ! (if_none . precondition_passes (& weak_bar)) ; } # [test] fn precondition_any () { let foo = ETag :: from_static ("\"foo\"") ; let if_none = IfNoneMatch :: any () ; assert ! (! if_none . precondition_passes (& foo)) ; } }
};
}
