// Generated macro for impl_704 (impl)
macro_rules! Depcrate_collectionimpl_704 {
() => {
// Module: crate::collection
// Provides: {"impl_704"}
// Dependencies: {}
impl SizeRange { # [doc = " Creates a `SizeBounds` from a `RangeInclusive<usize>`."] pub fn new (range : RangeInclusive < usize >) -> Self { range . into () } # [doc = " Merges self together with some other argument producing a product"] # [doc = " type expected by some implementations of `A: Arbitrary` in"] # [doc = " `A::Parameters`. This can be more ergonomic to work with and may"] # [doc = " help type inference."] pub fn with < X > (self , and : X) -> product_type ! [Self , X] { product_pack ! [self , and] } # [doc = " Merges self together with some other argument generated with a"] # [doc = " default value producing a product type expected by some"] # [doc = " implementations of `A: Arbitrary` in `A::Parameters`."] # [doc = " This can be more ergonomic to work with and may help type inference."] pub fn lift < X : Default > (self) -> product_type ! [Self , X] { self . with (Default :: default ()) } # [doc = " The lower bound of the range (inclusive)."] pub fn start (& self) -> usize { self . 0 . start } # [doc = " Extract the ends `[low, high]` of a `SizeRange`."] pub fn start_end_incl (& self) -> (usize , usize) { (self . start () , self . end_incl ()) } # [doc = " The upper bound of the range (inclusive)."] pub fn end_incl (& self) -> usize { self . 0 . end - 1 } # [doc = " The upper bound of the range (exclusive)."] pub fn end_excl (& self) -> usize { self . 0 . end } pub (crate) fn iter (& self) -> impl Iterator < Item = usize > { self . 0 . clone () . into_iter () } pub (crate) fn is_empty (& self) -> bool { self . start () == self . end_excl () } pub (crate) fn assert_nonempty (& self) { if self . is_empty () { panic ! ("Invalid use of empty size range. (hint: did you \
                 accidentally write {}..{} where you meant {}..={} \
                 somewhere?)" , self . start () , self . end_excl () , self . start () , self . end_excl ()) ; } } }
};
}
