// Generated macro for iter_after (function)
macro_rules! Depcrateiter_after {
() => {
// Module: crate
// Provides: {"iter_after"}
// Dependencies: {}
# [inline (always)] fn iter_after < 'a , 'b , I , J > (mut iter : I , mut prefix : J) -> Option < I > where I : Iterator < Item = Component < 'a > > + Clone , J : Iterator < Item = Component < 'b > > , { loop { let mut iter_next = iter . clone () ; match (iter_next . next () , prefix . next ()) { (Some (x) , Some (y)) if x == y => () , (Some (_) | None , Some (_)) => return None , (Some (_) | None , None) => return Some (iter) , } iter = iter_next ; } }
};
}
