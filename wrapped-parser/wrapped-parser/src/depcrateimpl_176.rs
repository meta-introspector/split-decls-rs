// Generated macro for impl_176 (impl)
macro_rules! Depcrateimpl_176 {
() => {
// Module: crate
// Provides: {"impl_176"}
// Dependencies: {}
impl Iterator for ErrorPositions { type Item = Pos ; fn next (& mut self) -> Option < Self :: Item > { match self . 0 { ErrorPositionsInner :: Two (a , b) => { self . 0 = ErrorPositionsInner :: One (b) ; Some (a) } ErrorPositionsInner :: One (a) => { self . 0 = ErrorPositionsInner :: None ; Some (a) } ErrorPositionsInner :: None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
