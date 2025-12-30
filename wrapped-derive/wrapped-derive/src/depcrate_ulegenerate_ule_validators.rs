// Generated macro for generate_ule_validators (function)
macro_rules! Depcrate_ulegenerate_ule_validators {
() => {
// Module: crate::ule
// Provides: {"generate_ule_validators"}
// Dependencies: {}
# [doc = " Given an slice over ULE struct fields, returns code validating that a slice variable `bytes` contains valid instances of those ULE types"] # [doc = " in order, plus the byte offset of any remaining unvalidated bytes. ULE types should not have any remaining bytes, but VarULE types will since"] # [doc = " the last field is the unsized one."] pub (crate) fn generate_ule_validators (fields : & [FieldInfo] ,) -> (TokenStream2 , syn :: Ident) { utils :: generate_per_field_offsets (fields , false , | field , prev_offset_ident , size_ident | { let ty = & field . field . ty ; quote ! { if let Some (bytes) = bytes . get (# prev_offset_ident .. # prev_offset_ident + # size_ident) { <# ty as zerovec :: ule :: ULE >:: validate_bytes (bytes) ?; } else { return Err (zerovec :: ule :: UleError :: parse ::< Self > ()) ; } } }) }
};
}
