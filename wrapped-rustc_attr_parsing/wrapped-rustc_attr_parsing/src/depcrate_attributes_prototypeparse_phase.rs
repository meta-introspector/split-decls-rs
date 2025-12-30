// Generated macro for parse_phase (function)
macro_rules! Depcrate_attributes_prototypeparse_phase {
() => {
// Module: crate::attributes::prototype
// Provides: {"parse_phase"}
// Dependencies: {}
fn parse_phase < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , phase : Option < (Symbol , Span) > , failed : & mut bool ,) -> Option < (MirPhase , Span) > { let (phase , span) = phase ? ; let phase = match phase { sym :: initial => MirPhase :: Initial , sym :: post_cleanup => MirPhase :: PostCleanup , sym :: optimized => MirPhase :: Optimized , _ => { cx . expected_specific_argument (span , & [sym :: initial , sym :: post_cleanup , sym :: optimized]) ; * failed = true ; return None ; } } ; Some ((phase , span)) }
};
}
