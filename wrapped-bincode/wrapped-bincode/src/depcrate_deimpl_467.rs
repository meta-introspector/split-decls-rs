// Generated macro for impl_467 (impl)
macro_rules! Depcrate_deimpl_467 {
() => {
// Module: crate::de
// Provides: {"impl_467"}
// Dependencies: {}
impl < 'de , T > BorrowDecoder < 'de > for & mut T where T : BorrowDecoder < 'de > , { type BR = T :: BR ; fn borrow_reader (& mut self) -> & mut Self :: BR { T :: borrow_reader (self) } }
};
}
