// Generated macro for levenstein_not_1 (function)
macro_rules! Depcrate_non_expressive_nameslevenstein_not_1 {
() => {
// Module: crate::non_expressive_names
// Provides: {"levenstein_not_1"}
// Dependencies: {}
# [doc = " Precondition: `a_name.chars().count() < b_name.chars().count()`."] # [must_use] fn levenstein_not_1 (a_name : & str , b_name : & str) -> bool { debug_assert ! (a_name . chars () . count () < b_name . chars () . count ()) ; let mut a_chars = a_name . chars () ; let mut b_chars = b_name . chars () ; while let (Some (a) , Some (b)) = (a_chars . next () , b_chars . next ()) { if a == b { continue ; } if let Some (b2) = b_chars . next () { return a != b2 || a_chars . ne (b_chars) ; } return true ; } true }
};
}
