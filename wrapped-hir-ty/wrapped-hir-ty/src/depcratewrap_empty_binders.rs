// Generated macro for wrap_empty_binders (function)
macro_rules! Depcratewrap_empty_binders {
() => {
// Module: crate
// Provides: {"wrap_empty_binders"}
// Dependencies: {}
pub (crate) fn wrap_empty_binders < T > (value : T) -> Binders < T > where T : TypeFoldable < Interner > + HasInterner < Interner = Interner > , { Binders :: empty (Interner , value . shifted_in_from (Interner , DebruijnIndex :: ONE)) }
};
}
