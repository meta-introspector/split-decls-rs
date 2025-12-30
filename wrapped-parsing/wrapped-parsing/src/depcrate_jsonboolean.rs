// Generated macro for boolean (function)
macro_rules! Depcrate_jsonboolean {
() => {
// Module: crate::json
// Provides: {"boolean"}
// Dependencies: {}
fn boolean (input : & str) -> IResult < & str , bool > { alt ((value (false , tag ("false")) , value (true , tag ("true")))) . parse (input) }
};
}
