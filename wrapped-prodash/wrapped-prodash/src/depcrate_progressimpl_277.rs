// Generated macro for impl_277 (impl)
macro_rules! Depcrate_progressimpl_277 {
() => {
// Module: crate::progress
// Provides: {"impl_277"}
// Dependencies: {}
impl std :: hash :: Hash for Value { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let Self { step , done_at , unit , state : our_state , } = self ; done_at . hash (state) ; unit . hash (state) ; our_state . hash (state) ; step . load (Ordering :: Relaxed) . hash (state) ; } }
};
}
