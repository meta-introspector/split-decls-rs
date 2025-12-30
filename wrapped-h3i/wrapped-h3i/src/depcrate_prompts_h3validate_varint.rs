// Generated macro for validate_varint (function)
macro_rules! Depcrate_prompts_h3validate_varint {
() => {
// Module: crate::prompts::h3
// Provides: {"validate_varint"}
// Dependencies: {}
fn validate_varint (id : & str) -> SuggestionResult < Validation > { let x = id . parse :: < u64 > () ; match x { Ok (v) => if v >= u64 :: pow (2 , 62) { return Ok (Validation :: Invalid (ErrorMessage :: Default)) ; } , Err (_) => { return Ok (Validation :: Invalid (ErrorMessage :: Default)) ; } , } Ok (Validation :: Valid) }
};
}
