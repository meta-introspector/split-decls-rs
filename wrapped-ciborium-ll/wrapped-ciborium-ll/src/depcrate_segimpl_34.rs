// Generated macro for impl_34 (impl)
macro_rules! Depcrate_segimpl_34 {
() => {
// Module: crate::seg
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'r , R : Read , P : Parser > Segment < 'r , R , P > { # [doc = " Gets the number of unprocessed bytes"] # [inline] pub fn left (& self) -> usize { self . unread + self . parser . saved () } # [doc = " Gets the next parsed chunk within the segment"] # [doc = ""] # [doc = " Returns `Ok(None)` when all chunks have been read."] # [inline] pub fn pull < 'a > (& mut self , buffer : & 'a mut [u8] ,) -> Result < Option < & 'a P :: Item > , Error < R :: Error > > { use core :: cmp :: min ; let prev = self . parser . saved () ; match self . unread { 0 if prev == 0 => return Ok (None) , 0 => return Err (Error :: Syntax (self . offset)) , _ => () , } let size = min (buffer . len () , prev + self . unread) ; let full = & mut buffer [.. size] ; let next = & mut full [min (size , prev) ..] ; self . reader . read_exact (next) ? ; self . unread -= next . len () ; self . parser . parse (full) . or (Err (Error :: Syntax (self . offset))) . map (Some) } }
};
}
