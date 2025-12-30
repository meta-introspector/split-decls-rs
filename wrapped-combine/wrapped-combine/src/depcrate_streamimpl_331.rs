// Generated macro for impl_331 (impl)
macro_rules! Depcrate_streamimpl_331 {
() => {
// Module: crate::stream
// Provides: {"impl_331"}
// Dependencies: {}
impl < Input : Iterator > StreamOnce for IteratorStream < Input > where Input :: Item : Clone + PartialEq , { type Token = Input :: Item ; type Range = Input :: Item ; type Position = () ; type Error = UnexpectedParse ; # [inline] fn uncons (& mut self) -> Result < Self :: Token , StreamErrorFor < Self > > { match self . next () { Some (x) => Ok (x) , None => Err (UnexpectedParse :: Eoi) , } } }
};
}
