// Generated macro for Context (struct)
macro_rules! Depcrate_outputContext {
() => {
// Module: crate::output
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Subset of `AllFacts` dedicated to borrow checking, and data ready to use by the variants"] struct Context < 'ctx , T : FactTypes > { origin_live_on_entry : Relation < (T :: Origin , T :: Point) > , loan_invalidated_at : Relation < (T :: Loan , T :: Point) > , subset_base : & 'ctx Vec < (T :: Origin , T :: Origin , T :: Point) > , loan_issued_at : & 'ctx Vec < (T :: Origin , T :: Loan , T :: Point) > , loan_killed_at : Relation < (T :: Loan , T :: Point) > , known_contains : Relation < (T :: Origin , T :: Loan) > , placeholder_origin : Relation < (T :: Origin , ()) > , placeholder_loan : Relation < (T :: Loan , T :: Origin) > , known_placeholder_subset : Relation < (T :: Origin , T :: Origin) > , cfg_edge : Relation < (T :: Point , T :: Point) > , # [allow (dead_code)] potential_errors : Option < FxHashSet < T :: Loan > > , # [allow (dead_code)] potential_subset_errors : Option < Relation < (T :: Origin , T :: Origin) > > , }
};
}
