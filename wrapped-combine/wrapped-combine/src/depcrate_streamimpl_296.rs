// Generated macro for impl_296 (impl)
macro_rules! Depcrate_streamimpl_296 {
() => {
// Module: crate::stream
// Provides: {"impl_296"}
// Dependencies: {}
impl < 'a > RangeStreamOnce for & 'a str { fn uncons_while < F > (& mut self , f : F) -> Result < & 'a str , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { Ok (str_uncons_while (self , self . chars () , f)) } # [inline] fn uncons_while1 < F > (& mut self , mut f : F) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { let mut chars = self . chars () ; match chars . next () { Some (c) => { if ! f (c) { return PeekErr (Tracked :: from (StringStreamError :: UnexpectedParse)) ; } } None => return PeekErr (Tracked :: from (StringStreamError :: Eoi)) , } CommitOk (str_uncons_while (self , chars , f)) } # [inline] fn uncons_range (& mut self , size : usize) -> Result < & 'a str , StreamErrorFor < Self > > { fn is_char_boundary (s : & str , index : usize) -> bool { if index == s . len () { return true ; } match s . as_bytes () . get (index) { None => false , Some (b) => ! (128 ..= 192) . contains (b) , } } if size <= self . len () { if is_char_boundary (self , size) { let (result , remaining) = self . split_at (size) ; * self = remaining ; Ok (result) } else { Err (StringStreamError :: CharacterBoundary) } } else { Err (StringStreamError :: Eoi) } } # [inline] fn distance (& self , end : & Self) -> usize { self . position () . 0 - end . position () . 0 } fn range (& self) -> Self :: Range { self } }
};
}
