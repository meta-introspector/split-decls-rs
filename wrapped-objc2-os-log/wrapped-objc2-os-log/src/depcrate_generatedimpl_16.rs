// Generated macro for impl_16 (impl)
macro_rules! Depcrate_generatedimpl_16 {
() => {
// Module: crate::generated
// Provides: {"impl_16"}
// Dependencies: {}
impl OSLogEntry { extern_methods ! (# [doc = " The fully formatted message for the entry."] # [unsafe (method (composedMessage))] # [unsafe (method_family = none)] pub unsafe fn composedMessage (& self) -> Retained < NSString >; # [doc = " The timestamp of the entry."] # [unsafe (method (date))] # [unsafe (method_family = none)] pub unsafe fn date (& self) -> Retained < NSDate >; # [doc = " This entry's storage tag. See OSLogEntryStoreCategory."] # [unsafe (method (storeCategory))] # [unsafe (method_family = none)] pub unsafe fn storeCategory (& self) -> OSLogEntryStoreCategory ;) ; }
};
}
