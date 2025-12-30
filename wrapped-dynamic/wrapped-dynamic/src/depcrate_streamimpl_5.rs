// Generated macro for impl_5 (impl)
macro_rules! Depcrate_streamimpl_5 {
() => {
// Module: crate::stream
// Provides: {"impl_5"}
// Dependencies: {}
impl < 'sval , R : sval :: Stream < 'sval > > private :: EraseStream < 'sval > for R { fn erase_stream_ref (& self) -> crate :: private :: Erased < & dyn private :: DispatchStream < 'sval > > { crate :: private :: Erased (self) } fn erase_stream (& mut self) -> crate :: private :: Erased < & mut dyn private :: DispatchStream < 'sval > > { crate :: private :: Erased (self) } }
};
}
