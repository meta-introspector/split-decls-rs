// Generated macro for DEFAULT_LAMPORTS_PER_BYTE_YEAR (const)
macro_rules! DepcrateDEFAULT_LAMPORTS_PER_BYTE_YEAR {
() => {
// Module: crate
// Provides: {"DEFAULT_LAMPORTS_PER_BYTE_YEAR"}
// Dependencies: {}
# [doc = " Default rental rate in lamports/byte-year."] # [doc = ""] # [doc = " This calculation is based on:"] # [doc = " - 10^9 lamports per SOL"] # [doc = " - $1 per SOL"] # [doc = " - $0.01 per megabyte day"] # [doc = " - $3.65 per megabyte year"] # [deprecated (since = "3.1.0" , note = "The concept of rent no longer exists, only rent-exemption. Use `DEFAULT_LAMPORTS_PER_BYTE` instead")] pub const DEFAULT_LAMPORTS_PER_BYTE_YEAR : u64 = 1_000_000_000 / 100 * 365 / (1024 * 1024) ;
};
}
