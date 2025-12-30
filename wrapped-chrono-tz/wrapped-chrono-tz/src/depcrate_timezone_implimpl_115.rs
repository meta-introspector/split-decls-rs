// Generated macro for impl_115 (impl)
macro_rules! Depcrate_timezone_implimpl_115 {
() => {
// Module: crate::timezone_impl
// Provides: {"impl_115"}
// Dependencies: {}
impl Span { fn contains (& self , x : i64) -> bool { match (self . begin , self . end) { (Some (a) , Some (b)) if a <= x && x < b => true , (Some (a) , None) if a <= x => true , (None , Some (b)) if b > x => true , (None , None) => true , _ => false , } } fn cmp (& self , x : i64) -> Ordering { match (self . begin , self . end) { (Some (a) , Some (b)) if a <= x && x < b => Ordering :: Equal , (Some (a) , Some (b)) if a <= x && b <= x => Ordering :: Less , (Some (_) , Some (_)) => Ordering :: Greater , (Some (a) , None) if a <= x => Ordering :: Equal , (Some (_) , None) => Ordering :: Greater , (None , Some (b)) if b <= x => Ordering :: Less , (None , Some (_)) => Ordering :: Equal , (None , None) => Ordering :: Equal , } } }
};
}
