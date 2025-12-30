// Generated macro for impl_293 (impl)
macro_rules! Depcrate_streamimpl_293 {
() => {
// Module: crate::stream
// Provides: {"impl_293"}
// Dependencies: {}
impl < 'a > StreamOnce for & 'a str { type Token = char ; type Range = & 'a str ; type Position = PointerOffset < str > ; type Error = StringStreamError ; # [inline] fn uncons (& mut self) -> Result < char , StreamErrorFor < Self > > { let mut chars = self . chars () ; match chars . next () { Some (c) => { * self = chars . as_str () ; Ok (c) } None => Err (StringStreamError :: Eoi) , } } }
};
}
