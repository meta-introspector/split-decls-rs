// Generated macro for impl_172 (impl)
macro_rules! Depcrate_propertiesimpl_172 {
() => {
// Module: crate::properties
// Provides: {"impl_172"}
// Dependencies: {}
impl Arbitrary for Repeater { fn arbitrary < G : Gen > (g : & mut G) -> Repeater { use Repeater :: * ; match g . gen_range (0 , 4) { 0 => ZeroOrOne , 1 => ZeroOrMore , 2 => OneOrMore , 3 => { use std :: cmp :: { max , min } ; let n1 = Arbitrary :: arbitrary (g) ; let n2 = Arbitrary :: arbitrary (g) ; Range { min : min (n1 , n2) , max : if g . gen () { None } else { Some (max (n1 , n2)) } , } } , _ => unreachable ! () , } } fn shrink (& self) -> Box < Iterator < Item = Repeater > > { use Repeater :: * ; match * self { ZeroOrOne | ZeroOrMore | OneOrMore => Box :: new (None . into_iter ()) , Range { min , max } => { Box :: new ((min , max) . shrink () . map (| (min , max) | Range { min : min , max : max })) } } } }
};
}
