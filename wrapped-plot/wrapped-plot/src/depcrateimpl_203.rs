// Generated macro for impl_203 (impl)
macro_rules! Depcrateimpl_203 {
() => {
// Module: crate
// Provides: {"impl_203"}
// Dependencies: {}
impl Axis { fn next (self) -> Option < Axis > { use crate :: Axis :: * ; match self { BottomX => Some (LeftY) , LeftY => Some (RightY) , RightY => Some (TopX) , TopX => None , } } }
};
}
