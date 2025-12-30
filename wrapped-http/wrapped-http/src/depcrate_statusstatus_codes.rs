// Generated macro for status_codes (macro)
macro_rules! Depcrate_statusstatus_codes {
() => {
// Module: crate::status
// Provides: {"status_codes"}
// Dependencies: {}
macro_rules ! status_codes { ($ ($ (# [$ docs : meta]) * ($ num : expr , $ konst : ident , $ phrase : expr) ;) +) => { impl StatusCode { $ ($ (# [$ docs]) * pub const $ konst : StatusCode = StatusCode (unsafe { NonZeroU16 :: new_unchecked ($ num) }) ;) + } fn canonical_reason (num : u16) -> Option <&'static str > { match num { $ ($ num => Some ($ phrase) ,) + _ => None } } } }
};
}
