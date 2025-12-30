// Generated macro for ThreadExtraState (struct)
macro_rules! Depcrate_concurrency_data_raceThreadExtraState {
() => {
// Module: crate::concurrency::data_race
// Provides: {"ThreadExtraState"}
// Dependencies: {}
# [doc = " Extra metadata associated with a thread."] # [derive (Debug , Clone , Default)] struct ThreadExtraState { # [doc = " The current vector index in use by the"] # [doc = " thread currently, this is set to None"] # [doc = " after the vector index has been re-used"] # [doc = " and hence the value will never need to be"] # [doc = " read during data-race reporting."] vector_index : Option < VectorIdx > , # [doc = " Thread termination vector clock, this"] # [doc = " is set on thread termination and is used"] # [doc = " for joining on threads since the vector_index"] # [doc = " may be re-used when the join operation occurs."] termination_vector_clock : Option < VClock > , }
};
}
