// Generated macro for State (enum)
macro_rules! Depcrate_stream_foldState {
() => {
// Module: crate::stream::fold
// Provides: {"State"}
// Dependencies: {}
enum State < T , Fut > { # [doc = " Placeholder state when doing work"] Empty , # [doc = " Ready to process the next stream item; current accumulator is the `T`"] Ready (T) , # [doc = " Working on a future the process the previous stream item"] Processing (Fut) , }
};
}
