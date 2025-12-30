// Generated macro for macro_334 (macro)
macro_rules! Depcrate_fut_futuremacro_334 {
() => {
// Module: crate::fut::future
// Provides: {"macro_334"}
// Dependencies: {}
pin_project ! { pub struct FutureWrap < F , A > where F : Future , A : Actor , { # [pin] fut : F , _act : PhantomData < A > } }
};
}
