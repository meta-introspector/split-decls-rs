// Generated macro for impl_150 (impl)
macro_rules! Depcrate_ext_seimpl_150 {
() => {
// Module: crate::ext::se
// Provides: {"impl_150"}
// Dependencies: {}
impl ExtFieldSerializer { # [inline] const fn new () -> Self { Self { tag : None , binary : None , } } fn value (self) -> Result < Value , Error > { match (self . tag , self . binary) { (Some (tag) , Some (binary)) => Ok (Value :: Ext (tag , binary)) , (Some (_) , None) => Err (< Error as ser :: Error > :: custom ("expected i8 and bytes")) , (None , Some (_)) => Err (< Error as ser :: Error > :: custom ("expected i8 and bytes")) , (None , None) => Err (< Error as ser :: Error > :: custom ("expected i8 and bytes")) , } } }
};
}
