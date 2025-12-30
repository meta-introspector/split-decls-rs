// Generated macro for impl_2094 (impl)
macro_rules! Depcrate_compat_compat03as01impl_2094 {
() => {
// Module: crate::compat::compat03as01
// Provides: {"impl_2094"}
// Dependencies: {}
impl < St > Stream01 for Compat < St > where St : TryStream03 + Unpin , { type Item = St :: Ok ; type Error = St :: Error ; fn poll (& mut self) -> Poll01 < Option < Self :: Item > , Self :: Error > { with_context (self , | inner , cx | match inner . try_poll_next (cx) ? { task03 :: Poll :: Ready (None) => Ok (Async01 :: Ready (None)) , task03 :: Poll :: Ready (Some (t)) => Ok (Async01 :: Ready (Some (t))) , task03 :: Poll :: Pending => Ok (Async01 :: NotReady) , }) } }
};
}
