// Generated macro for impl_177 (impl)
macro_rules! Depcrateimpl_177 {
() => {
// Module: crate
// Provides: {"impl_177"}
// Dependencies: {}
impl DoubleEndedIterator for ErrorPositions { fn next_back (& mut self) -> Option < Self :: Item > { match self . 0 { ErrorPositionsInner :: Two (a , b) => { self . 0 = ErrorPositionsInner :: One (a) ; Some (b) } ErrorPositionsInner :: One (a) => { self . 0 = ErrorPositionsInner :: None ; Some (a) } ErrorPositionsInner :: None => None , } } }
};
}
