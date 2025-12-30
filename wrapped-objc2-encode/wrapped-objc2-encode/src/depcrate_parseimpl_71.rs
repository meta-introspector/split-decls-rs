// Generated macro for impl_71 (impl)
macro_rules! Depcrate_parseimpl_71 {
() => {
// Module: crate::parse
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'a > Parser < 'a > { pub (crate) fn new (data : & 'a str) -> Self { Self { split_point : 0 , data , } } pub (crate) fn remaining (& self) -> & 'a str { & self . data [self . split_point ..] } fn peek (& self) -> Result < u8 > { self . try_peek () . ok_or (ErrorKind :: UnexpectedEnd) } fn try_peek (& self) -> Option < u8 > { self . data . as_bytes () . get (self . split_point) . copied () } fn try_peek2 (& self) -> Option < (u8 , u8) > { let bytes = self . data . as_bytes () ; Some ((* bytes . get (self . split_point) ? , * bytes . get (self . split_point + 1) ? ,)) } fn advance (& mut self) { self . split_point += 1 ; } fn rollback (& mut self) { self . split_point -= 1 ; } fn consume_while (& mut self , mut condition : impl FnMut (u8) -> bool) { while let Some (b) = self . try_peek () { if condition (b) { self . advance () ; } else { break ; } } } pub (crate) fn is_empty (& self) -> bool { self . try_peek () . is_none () } pub (crate) fn expect_empty (& self) -> Result < () > { if self . is_empty () { Ok (()) } else { Err (ErrorKind :: NotAllConsumed) } } }
};
}
