// Generated macro for mk_async_fn (macro)
macro_rules! Depcratemk_async_fn {
() => {
// Module: crate
// Provides: {"mk_async_fn"}
// Dependencies: {}
macro_rules ! mk_async_fn { ($ f : ident $ g : ident) => { async fn $ g () -> i32 { $ f () . await ; $ f () . await ; $ f () . await } } }
};
}
