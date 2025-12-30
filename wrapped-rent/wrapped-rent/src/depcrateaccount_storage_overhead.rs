// Generated macro for ACCOUNT_STORAGE_OVERHEAD (const)
macro_rules! DepcrateACCOUNT_STORAGE_OVERHEAD {
() => {
// Module: crate
// Provides: {"ACCOUNT_STORAGE_OVERHEAD"}
// Dependencies: {}
# [doc = " Account storage overhead for calculation of base rent."] # [doc = ""] # [doc = " This is the number of bytes required to store an account with no data. It is"] # [doc = " added to an accounts data length when calculating [`Rent::minimum_balance`]."] pub const ACCOUNT_STORAGE_OVERHEAD : u64 = 128 ;
};
}
