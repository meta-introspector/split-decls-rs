// Generated macro for AsyncBencher (struct)
macro_rules! Depcrate_bencherAsyncBencher {
() => {
// Module: crate::bencher
// Provides: {"AsyncBencher"}
// Dependencies: {}
# [doc = " Async/await variant of [`Bencher`]."] # [cfg (feature = "async")] pub struct AsyncBencher < 'a , 'b , A : AsyncExecutor , M : Measurement = WallTime > { b : & 'b mut Bencher < 'a , M > , runner : A , }
};
}
