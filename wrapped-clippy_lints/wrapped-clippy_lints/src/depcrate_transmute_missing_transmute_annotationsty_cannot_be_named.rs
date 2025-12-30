// Generated macro for ty_cannot_be_named (function)
macro_rules! Depcrate_transmute_missing_transmute_annotationsty_cannot_be_named {
() => {
// Module: crate::transmute::missing_transmute_annotations
// Provides: {"ty_cannot_be_named"}
// Dependencies: {}
fn ty_cannot_be_named (ty : Ty < '_ >) -> bool { matches ! (ty . kind () , ty :: Alias (ty :: AliasTyKind :: Opaque | ty :: AliasTyKind :: Inherent , _)) }
};
}
