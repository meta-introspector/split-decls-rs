// Generated macro for is_stable (function)
macro_rules! Depcrate_std_instead_of_coreis_stable {
() => {
// Module: crate::std_instead_of_core
// Provides: {"is_stable"}
// Dependencies: {}
# [doc = " Checks if all ancestors of `def_id` meet `msrv` to avoid linting [unstable moves](https://github.com/rust-lang/rust/pull/95956)"] # [doc = " or now stable moves that were once unstable."] # [doc = ""] # [doc = " Does not catch individually moved items"] fn is_stable (cx : & LateContext < '_ > , mut def_id : DefId , msrv : Msrv) -> bool { loop { if let Some (stability) = cx . tcx . lookup_stability (def_id) && let StabilityLevel :: Stable { since , allowed_through_unstable_modules : None , } = stability . level { let stable = match since { StableSince :: Version (v) => msrv . meets (cx , v) , StableSince :: Current => msrv . current (cx) . is_none () , StableSince :: Err (_) => false , } ; if ! stable { return false ; } } match cx . tcx . opt_parent (def_id) { Some (parent) => def_id = parent , None => return true , } } }
};
}
