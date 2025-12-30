// Generated macro for impl_131 (impl)
macro_rules! Depcrateimpl_131 {
() => {
// Module: crate
// Provides: {"impl_131"}
// Dependencies: {}
impl fmt :: Display for Repeater { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { ZeroOrOne => write ! (f , "?") , ZeroOrMore => write ! (f , "*") , OneOrMore => write ! (f , "+") , Range { min : s , max : None } => write ! (f , "{{{},}}" , s) , Range { min : s , max : Some (e) } if s == e => write ! (f , "{{{}}}" , s) , Range { min : s , max : Some (e) } => write ! (f , "{{{}, {}}}" , s , e) , } } }
};
}
