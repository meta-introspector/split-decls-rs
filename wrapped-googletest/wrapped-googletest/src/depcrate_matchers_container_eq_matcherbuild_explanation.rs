// Generated macro for build_explanation (function)
macro_rules! Depcrate_matchers_container_eq_matcherbuild_explanation {
() => {
// Module: crate::matchers::container_eq_matcher
// Provides: {"build_explanation"}
// Dependencies: {}
fn build_explanation < T : Debug , U : Debug > (missing : Vec < T > , unexpected : Vec < U >) -> String { match (missing . len () , unexpected . len ()) { (0 , 0) => "which contains all the elements" . to_string () , (0 , 1) => format ! ("which contains the unexpected element {:?}" , unexpected [0]) , (0 , _) => format ! ("which contains the unexpected elements {unexpected:?}" ,) , (1 , 0) => format ! ("which is missing the element {:?}" , missing [0]) , (1 , 1) => { format ! ("which is missing the element {:?} and contains the unexpected element {:?}" , missing [0] , unexpected [0]) } (1 , _) => { format ! ("which is missing the element {:?} and contains the unexpected elements {unexpected:?}" , missing [0]) } (_ , 0) => format ! ("which is missing the elements {missing:?}") , (_ , 1) => { format ! ("which is missing the elements {missing:?} and contains the unexpected element {:?}" , unexpected [0]) } (_ , _) => { format ! ("which is missing the elements {missing:?} and contains the unexpected elements {unexpected:?}" ,) } } }
};
}
