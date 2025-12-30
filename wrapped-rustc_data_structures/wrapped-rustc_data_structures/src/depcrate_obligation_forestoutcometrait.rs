// Generated macro for OutcomeTrait (trait)
macro_rules! Depcrate_obligation_forestOutcomeTrait {
() => {
// Module: crate::obligation_forest
// Provides: {"OutcomeTrait"}
// Dependencies: {}
# [doc = " This trait allows us to have two different Outcome types:"] # [doc = "  - the normal one that does as little as possible"] # [doc = "  - one for tests that does some additional work and checking"] pub trait OutcomeTrait { type Error ; type Obligation ; fn new () -> Self ; fn record_completed (& mut self , outcome : & Self :: Obligation) ; fn record_error (& mut self , error : Self :: Error) ; }
};
}
