// Generated macro for impl_269 (impl)
macro_rules! Depcrateimpl_269 {
() => {
// Module: crate
// Provides: {"impl_269"}
// Dependencies: {}
impl TryFrom < usize > for HeadingLevel { type Error = InvalidHeadingLevel ; fn try_from (value : usize) -> Result < Self , Self :: Error > { match value { 1 => Ok (Self :: H1) , 2 => Ok (Self :: H2) , 3 => Ok (Self :: H3) , 4 => Ok (Self :: H4) , 5 => Ok (Self :: H5) , 6 => Ok (Self :: H6) , _ => Err (InvalidHeadingLevel (value)) , } } }
};
}
