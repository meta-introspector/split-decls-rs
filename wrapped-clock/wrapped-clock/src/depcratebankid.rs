// Generated macro for BankId (type)
macro_rules! DepcrateBankId {
() => {
// Module: crate
// Provides: {"BankId"}
// Dependencies: {}
# [doc = " Uniquely distinguishes every version of a slot."] # [doc = ""] # [doc = " The `BankId` is unique even if the slot number of two different slots is the"] # [doc = " same. This can happen in the case of e.g. duplicate slots."] pub type BankId = u64 ;
};
}
