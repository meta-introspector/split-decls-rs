// Generated macro for impl_51 (impl)
macro_rules! Depcrate_parserimpl_51 {
() => {
// Module: crate::parser
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a > Bumpable for & 'a str { fn match_end (self , p : & Parser) -> usize { let mut search = self . chars () ; let mut rest = p . chars () ; let mut count = 0 ; loop { match (rest . next () , search . next ()) { (Some (c1) , Some (c2)) if c1 == c2 => count = rest . cur , (_ , None) => return count , _ => return 0 , } } } }
};
}
