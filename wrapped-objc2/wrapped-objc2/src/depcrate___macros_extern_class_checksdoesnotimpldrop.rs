// Generated macro for DoesNotImplDrop (trait)
macro_rules! Depcrate___macros_extern_class_checksDoesNotImplDrop {
() => {
// Module: crate::__macros::extern_class::checks
// Provides: {"DoesNotImplDrop"}
// Dependencies: {}
# [doc = " Check that class does not implement `Drop`."] # [doc = ""] # [doc = " This is not needed for soundness, it's just a nice footgun to avoid (since"] # [doc = " it wouldn't ever get called)."] # [doc = ""] # [doc = " Check implemented using type inference:"] # [doc = " let _ = <MyType as DoesNotImplDrop<_>>::check"] pub trait DoesNotImplDrop < Inferred > { fn check () { } }
};
}
