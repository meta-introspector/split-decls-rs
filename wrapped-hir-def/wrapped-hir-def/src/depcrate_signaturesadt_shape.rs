// Generated macro for adt_shape (function)
macro_rules! Depcrate_signaturesadt_shape {
() => {
// Module: crate::signatures
// Provides: {"adt_shape"}
// Dependencies: {}
# [inline] fn adt_shape (adt_kind : ast :: StructKind) -> FieldsShape { match adt_kind { ast :: StructKind :: Record (_) => FieldsShape :: Record , ast :: StructKind :: Tuple (_) => FieldsShape :: Tuple , ast :: StructKind :: Unit => FieldsShape :: Unit , } }
};
}
