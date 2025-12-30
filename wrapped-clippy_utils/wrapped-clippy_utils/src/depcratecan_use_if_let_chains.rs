// Generated macro for can_use_if_let_chains (function)
macro_rules! Depcratecan_use_if_let_chains {
() => {
// Module: crate
// Provides: {"can_use_if_let_chains"}
// Dependencies: {}
# [doc = " Checks if the chosen edition and `msrv` allows using `if let` chains."] pub fn can_use_if_let_chains (cx : & LateContext < '_ > , msrv : Msrv) -> bool { cx . tcx . sess . edition () . at_least_rust_2024 () && msrv . meets (cx , msrvs :: LET_CHAINS) }
};
}
