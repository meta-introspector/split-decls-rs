// Generated macro for Output (struct)
macro_rules! Depcrate_outputOutput {
() => {
// Module: crate::output
// Provides: {"Output"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct Output < T : FactTypes > { pub errors : FxHashMap < T :: Point , Vec < T :: Loan > > , pub subset_errors : FxHashMap < T :: Point , BTreeSet < (T :: Origin , T :: Origin) > > , pub move_errors : FxHashMap < T :: Point , Vec < T :: Path > > , pub dump_enabled : bool , pub loan_live_at : FxHashMap < T :: Point , Vec < T :: Loan > > , pub origin_contains_loan_at : FxHashMap < T :: Point , BTreeMap < T :: Origin , BTreeSet < T :: Loan > > > , pub origin_contains_loan_anywhere : FxHashMap < T :: Origin , BTreeSet < T :: Loan > > , pub origin_live_on_entry : FxHashMap < T :: Point , Vec < T :: Origin > > , pub loan_invalidated_at : FxHashMap < T :: Point , Vec < T :: Loan > > , pub subset : FxHashMap < T :: Point , BTreeMap < T :: Origin , BTreeSet < T :: Origin > > > , pub subset_anywhere : FxHashMap < T :: Origin , BTreeSet < T :: Origin > > , pub var_live_on_entry : FxHashMap < T :: Point , Vec < T :: Variable > > , pub var_drop_live_on_entry : FxHashMap < T :: Point , Vec < T :: Variable > > , pub path_maybe_initialized_on_exit : FxHashMap < T :: Point , Vec < T :: Path > > , pub path_maybe_uninitialized_on_exit : FxHashMap < T :: Point , Vec < T :: Path > > , pub known_contains : FxHashMap < T :: Origin , BTreeSet < T :: Loan > > , pub var_maybe_partly_initialized_on_exit : FxHashMap < T :: Point , Vec < T :: Variable > > , }
};
}
