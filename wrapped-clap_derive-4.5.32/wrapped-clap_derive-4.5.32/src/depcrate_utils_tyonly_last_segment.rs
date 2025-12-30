// Generated macro for only_last_segment (function)
macro_rules! Depcrate_utils_tyonly_last_segment {
() => {
// Module: crate::utils::ty
// Provides: {"only_last_segment"}
// Dependencies: {}
fn only_last_segment (mut ty : & Type) -> Option < & PathSegment > { while let Type :: Group (syn :: TypeGroup { elem , .. }) = ty { ty = elem ; } match ty { Type :: Path (TypePath { qself : None , path : Path { leading_colon : None , segments , } , }) => only_one (segments . iter ()) , _ => None , } }
};
}
