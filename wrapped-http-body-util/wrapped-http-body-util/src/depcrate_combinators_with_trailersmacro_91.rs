// Generated macro for macro_91 (macro)
macro_rules! Depcrate_combinators_with_trailersmacro_91 {
() => {
// Module: crate::combinators::with_trailers
// Provides: {"macro_91"}
// Dependencies: {}
pin_project ! { # [project = StateProj] enum State < T , F > { PollBody { # [pin] body : T , trailers : Option < F >, } , PollTrailers { # [pin] trailers : F , prev_trailers : Option < HeaderMap >, } , Done , } }
};
}
