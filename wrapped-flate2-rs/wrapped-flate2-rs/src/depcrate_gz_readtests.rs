// Generated macro for tests (module)
macro_rules! Depcrate_gz_readtests {
() => {
// Module: crate::gz::read
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: io :: { Cursor , ErrorKind , Read , Result , Write } ; use super :: GzDecoder ; # [derive (Debug)] pub struct BlockingCursor { pub cursor : Cursor < Vec < u8 > > , } impl BlockingCursor { pub fn new () -> BlockingCursor { BlockingCursor { cursor : Cursor :: new (Vec :: new ()) , } } pub fn set_position (& mut self , pos : u64) { self . cursor . set_position (pos) } } impl Write for BlockingCursor { fn write (& mut self , buf : & [u8]) -> Result < usize > { self . cursor . write (buf) } fn flush (& mut self) -> Result < () > { self . cursor . flush () } } impl Read for BlockingCursor { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { let r = self . cursor . read (buf) ; match r { Err (ref err) => { if err . kind () == ErrorKind :: UnexpectedEof { return Err (ErrorKind :: WouldBlock . into ()) ; } } Ok (0) => { return Err (ErrorKind :: WouldBlock . into ()) ; } Ok (_n) => { } } r } } # [test] fn blocked_partial_header_read () { let mut r = BlockingCursor :: new () ; let data = vec ! [1 , 2 , 3] ; match r . write_all (& data) { Ok (()) => { } _ => { panic ! ("Unexpected result for write_all") ; } } r . set_position (0) ; let mut decoder = GzDecoder :: new (r) ; let mut out = Vec :: with_capacity (7) ; match decoder . read (& mut out) { Err (e) => { assert_eq ! (e . kind () , ErrorKind :: WouldBlock) ; } _ => { panic ! ("Unexpected result for decoder.read") ; } } } }
};
}
