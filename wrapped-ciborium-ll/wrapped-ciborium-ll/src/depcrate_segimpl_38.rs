// Generated macro for impl_38 (impl)
macro_rules! Depcrate_segimpl_38 {
() => {
// Module: crate::seg
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'r , R : Read , P : Parser > Segments < 'r , R , P > { # [doc = " Gets the next segment in the stream"] # [doc = ""] # [doc = " Returns `Ok(None)` at the conclusion of the stream."] # [inline] pub fn pull < 'a > (& 'a mut self) -> Result < Option < Segment < 'a , R , P > > , Error < R :: Error > > { while self . state != State :: Finished { let offset = self . reader . offset () ; match self . reader . pull () ? { Header :: Break => { self . state = State :: Finished ; return Ok (None) ; } header => match (self . unwrap) (header) { Err (..) => return Err (Error :: Syntax (offset)) , Ok (None) => { if self . state == State :: Initial { self . state = State :: Continue ; } else { return Err (Error :: Syntax (offset)) ; } } Ok (Some (len)) => { if self . state == State :: Initial { self . state = State :: Finished ; } return Ok (Some (Segment { reader : self . reader , unread : len , offset , parser : P :: default () , })) ; } } , } } Ok (None) } }
};
}
