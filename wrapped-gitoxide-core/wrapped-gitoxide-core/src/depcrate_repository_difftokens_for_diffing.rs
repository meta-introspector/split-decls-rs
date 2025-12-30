// Generated macro for tokens_for_diffing (function)
macro_rules! Depcrate_repository_difftokens_for_diffing {
() => {
// Module: crate::repository::diff
// Provides: {"tokens_for_diffing"}
// Dependencies: {}
pub (crate) fn tokens_for_diffing (data : & [u8]) -> impl TokenSource < Token = & [u8] > { gix :: diff :: blob :: sources :: byte_lines (data) }
};
}
