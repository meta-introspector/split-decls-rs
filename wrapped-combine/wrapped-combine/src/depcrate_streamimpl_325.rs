// Generated macro for impl_325 (impl)
macro_rules! Depcrate_streamimpl_325 {
() => {
// Module: crate::stream
// Provides: {"impl_325"}
// Dependencies: {}
impl < 'a , T > StreamOnce for SliceStream < 'a , T > where T : PartialEq + 'a , { type Token = & 'a T ; type Range = & 'a [T] ; type Position = PointerOffset < [T] > ; type Error = UnexpectedParse ; # [inline] fn uncons (& mut self) -> Result < & 'a T , StreamErrorFor < Self > > { match self . 0 . split_first () { Some ((first , rest)) => { self . 0 = rest ; Ok (first) } None => Err (UnexpectedParse :: Eoi) , } } }
};
}
