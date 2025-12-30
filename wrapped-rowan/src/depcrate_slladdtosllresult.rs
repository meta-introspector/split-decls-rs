// Generated macro for AddToSllResult (enum)
macro_rules! Depcrate_sllAddToSllResult {
() => {
// Module: crate::sll
// Provides: {"AddToSllResult"}
// Dependencies: {}
pub (crate) enum AddToSllResult < 'a , E : Elem > { NoHead , EmptyHead (& 'a Cell < * const E >) , SmallerThanHead (& 'a Cell < * const E >) , SmallerThanNotHead (* const E) , AlreadyInSll (* const E) , }
};
}
