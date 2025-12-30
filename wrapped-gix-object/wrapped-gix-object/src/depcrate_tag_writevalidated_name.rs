// Generated macro for validated_name (function)
macro_rules! Depcrate_tag_writevalidated_name {
() => {
// Module: crate::tag::write
// Provides: {"validated_name"}
// Dependencies: {}
fn validated_name (name : & BStr) -> Result < & BStr , Error > { gix_validate :: tag :: name (name) ? ; if name [0] == b'-' { return Err (Error :: StartsWithDash) ; } Ok (name) }
};
}
