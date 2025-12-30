// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < B : BitBlock > Iterator for TwoBitPositions < '_ , B > { type Item = B ; fn next (& mut self) -> Option < B > { match (self . set . next () , self . other . next ()) { (Some (a) , Some (b)) => Some ((self . merge) (a , b)) , (Some (a) , None) => Some ((self . merge) (a , B :: zero ())) , (None , Some (b)) => Some ((self . merge) (B :: zero () , b)) , _ => None , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (first_lower_bound , first_upper_bound) = self . set . size_hint () ; let (second_lower_bound , second_upper_bound) = self . other . size_hint () ; let upper_bound = first_upper_bound . zip (second_upper_bound) ; let get_max = | (a , b) | cmp :: max (a , b) ; (cmp :: max (first_lower_bound , second_lower_bound) , upper_bound . map (get_max) ,) } }
};
}
