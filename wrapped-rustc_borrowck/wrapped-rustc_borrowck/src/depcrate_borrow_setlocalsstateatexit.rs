// Generated macro for LocalsStateAtExit (enum)
macro_rules! Depcrate_borrow_setLocalsStateAtExit {
() => {
// Module: crate::borrow_set
// Provides: {"LocalsStateAtExit"}
// Dependencies: {}
pub enum LocalsStateAtExit { AllAreInvalidated , SomeAreInvalidated { has_storage_dead_or_moved : DenseBitSet < Local > } , }
};
}
