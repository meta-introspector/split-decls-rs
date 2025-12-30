// Generated macro for ApproxEqRatio (trait)
macro_rules! Depcrate_ratioApproxEqRatio {
() => {
// Module: crate::ratio
// Provides: {"ApproxEqRatio"}
// Dependencies: {}
# [doc = " ApproxEqRatio is a trait for approximate equality comparisons bounding the ratio"] # [doc = " of the difference to the larger."] pub trait ApproxEqRatio : Div < Output = Self > + Sub < Output = Self > + Neg < Output = Self > + PartialOrd + Zero + Sized + Copy { # [doc = " This method tests if `self` and `other` are nearly equal by bounding the"] # [doc = " difference between them to some number much less than the larger of the two."] # [doc = " This bound is set as the ratio of the difference to the larger."] fn approx_eq_ratio (& self , other : & Self , ratio : Self) -> bool { if * self < Self :: zero () && * other > Self :: zero () { return false ; } if * self > Self :: zero () && * other < Self :: zero () { return false ; } match (* self == Self :: zero () , * other == Self :: zero ()) { (true , true) => return true , (true , false) => return false , (false , true) => return false , _ => { } } let (s , o) = if * self < Self :: zero () { (- * self , - * other) } else { (* self , * other) } ; let (smaller , larger) = if s < o { (s , o) } else { (o , s) } ; let difference : Self = larger . sub (smaller) ; let actual_ratio : Self = difference . div (larger) ; actual_ratio < ratio } # [doc = " This method tests if `self` and `other` are not nearly equal by bounding the"] # [doc = " difference between them to some number much less than the larger of the two."] # [doc = " This bound is set as the ratio of the difference to the larger."] # [inline] fn approx_ne_ratio (& self , other : & Self , ratio : Self) -> bool { ! self . approx_eq_ratio (other , ratio) } }
};
}
