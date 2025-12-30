// Generated macro for tokens_for_side (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilstokens_for_side {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"tokens_for_side"}
// Dependencies: {}
fn tokens_for_side < 'a > (side : Side , input : & 'a InternedInput < & [u8] > , current_tokens : & 'a [Token] ,) -> & 'a [Token] { match side { Side :: Current => current_tokens , Side :: Other => & input . after , Side :: Ancestor => & input . before , } }
};
}
