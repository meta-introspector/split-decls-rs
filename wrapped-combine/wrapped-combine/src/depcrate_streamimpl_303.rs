// Generated macro for impl_303 (impl)
macro_rules! Depcrate_streamimpl_303 {
() => {
// Module: crate::stream
// Provides: {"impl_303"}
// Dependencies: {}
impl < 'a , T > StreamOnce for & 'a [T] where T : Clone + PartialEq , { type Token = T ; type Range = & 'a [T] ; type Position = PointerOffset < [T] > ; type Error = UnexpectedParse ; # [inline] fn uncons (& mut self) -> Result < T , StreamErrorFor < Self > > { match self . split_first () { Some ((first , rest)) => { * self = rest ; Ok (first . clone ()) } None => Err (UnexpectedParse :: Eoi) , } } }
};
}
