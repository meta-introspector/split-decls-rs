// Generated macro for impl_14 (impl)
macro_rules! Depcrate_blockimpl_14 {
() => {
// Module: crate::block
// Provides: {"impl_14"}
// Dependencies: {}
impl TryFrom < & '_ LitStr > for BlockContents { type Error = syn :: Error ; fn try_from (s : & LitStr) -> Result < Self , Self :: Error > { let mut block_str = s . value () ; block_str . insert (0 , '{') ; block_str . push ('}') ; LitStr :: new (& block_str , s . span ()) . parse () . map (Self) } }
};
}
