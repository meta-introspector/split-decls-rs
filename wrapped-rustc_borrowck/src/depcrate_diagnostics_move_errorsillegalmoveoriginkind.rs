// Generated macro for IllegalMoveOriginKind (enum)
macro_rules! Depcrate_diagnostics_move_errorsIllegalMoveOriginKind {
() => {
// Module: crate::diagnostics::move_errors
// Provides: {"IllegalMoveOriginKind"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum IllegalMoveOriginKind < 'tcx > { # [doc = " Illegal move due to attempt to move from behind a reference."] BorrowedContent { # [doc = " The place the reference refers to: if erroneous code was trying to"] # [doc = " move from `(*x).f` this will be `*x`."] target_place : Place < 'tcx > , } , # [doc = " Illegal move due to attempt to move from field of an ADT that"] # [doc = " implements `Drop`. Rust maintains invariant that all `Drop`"] # [doc = " ADT's remain fully-initialized so that user-defined destructor"] # [doc = " can safely read from all of the ADT's fields."] InteriorOfTypeWithDestructor { container_ty : Ty < 'tcx > } , # [doc = " Illegal move due to attempt to move out of a slice or array."] InteriorOfSliceOrArray { ty : Ty < 'tcx > , is_index : bool } , }
};
}
