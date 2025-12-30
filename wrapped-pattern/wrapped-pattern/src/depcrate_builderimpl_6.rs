// Generated macro for impl_6 (impl)
macro_rules! Depcrate_builderimpl_6 {
() => {
// Module: crate::builder
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a , K > Iterator for Parser < 'a , K > where K : FromStr , K :: Err : fmt :: Debug , { type Item = Result < PatternItemCow < 'a , K > , PatternError > ; fn next (& mut self) -> Option < Self :: Item > { match self . try_next () { Ok (Some (pattern_token)) => Some (Ok (pattern_token . into ())) , Ok (None) => None , Err (_e) => Some (Err (PatternError :: InvalidPattern)) , } } }
};
}
