// Generated macro for AccountView (struct)
macro_rules! DepcrateAccountView {
() => {
// Module: crate
// Provides: {"AccountView"}
// Dependencies: {}
# [doc = " Wrapper struct for a `RuntimeAccount`."] # [doc = ""] # [doc = " This struct provides safe access to the data in a `RuntimeAccount`."] # [doc = " It is also used to track borrows of the account data, given that"] # [doc = " an account can be \"shared\" across multiple `AccountView` instances."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " - The `raw` pointer must be valid and point to memory containing a"] # [doc = "   `RuntimeAccount` struct, immediately followed by the account's data"] # [doc = "   region."] # [doc = " - The length of the account data must exactly match the value stored in"] # [doc = "   `RuntimeAccount::data_len`."] # [doc = ""] # [doc = " These conditions must always hold for any `AccountView` created from"] # [doc = " a raw pointer."] # [repr (C)] # [cfg_attr (feature = "copy" , derive (Copy))] # [derive (Clone , PartialEq , Eq , Debug)] pub struct AccountView { # [doc = " Raw (pointer to) account data."] # [doc = ""] # [doc = " Note that this is a pointer can be shared across multiple `AccountView`."] raw : * mut RuntimeAccount , }
};
}
