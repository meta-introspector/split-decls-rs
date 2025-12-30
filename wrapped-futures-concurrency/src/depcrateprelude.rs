// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " The futures concurrency prelude."] pub mod prelude { pub use super :: future :: FutureExt as _ ; pub use super :: stream :: StreamExt as _ ; pub use super :: future :: Join as _ ; pub use super :: future :: Race as _ ; pub use super :: future :: RaceOk as _ ; pub use super :: future :: TryJoin as _ ; pub use super :: stream :: Chain as _ ; pub use super :: stream :: IntoStream as _ ; pub use super :: stream :: Merge as _ ; pub use super :: stream :: Zip as _ ; # [cfg (feature = "alloc")] pub use super :: concurrent_stream :: { ConcurrentStream , FromConcurrentStream , IntoConcurrentStream , } ; }
};
}
