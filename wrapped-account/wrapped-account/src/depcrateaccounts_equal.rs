// Generated macro for accounts_equal (function)
macro_rules! Depcrateaccounts_equal {
() => {
// Module: crate
// Provides: {"accounts_equal"}
// Dependencies: {}
# [doc = " Compares two ReadableAccounts"] # [doc = ""] # [doc = " Returns true if accounts are essentially equivalent as in all fields are equivalent."] pub fn accounts_equal < T : ReadableAccount , U : ReadableAccount > (me : & T , other : & U) -> bool { me . lamports () == other . lamports () && me . executable () == other . executable () && me . rent_epoch () == other . rent_epoch () && me . owner () == other . owner () && me . data () == other . data () }
};
}
