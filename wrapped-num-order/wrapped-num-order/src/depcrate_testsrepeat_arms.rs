// Generated macro for repeat_arms (macro)
macro_rules! Depcrate_testsrepeat_arms {
() => {
// Module: crate::tests
// Provides: {"repeat_arms"}
// Dependencies: {}
macro_rules ! repeat_arms { ($ e : expr ; $ v : ident => $ arm : expr) => { match $ e { N :: u8 ($ v) => $ arm , N :: u16 ($ v) => $ arm , N :: u32 ($ v) => $ arm , N :: u64 ($ v) => $ arm , N :: u128 ($ v) => $ arm , N :: usize ($ v) => $ arm , N :: i8 ($ v) => $ arm , N :: i16 ($ v) => $ arm , N :: i32 ($ v) => $ arm , N :: i64 ($ v) => $ arm , N :: i128 ($ v) => $ arm , N :: isize ($ v) => $ arm , N :: f32 ($ v) => $ arm , N :: f64 ($ v) => $ arm , # [cfg (feature = "num-bigint")] N :: ubig ($ v) => $ arm , # [cfg (feature = "num-bigint")] N :: ibig ($ v) => $ arm , # [cfg (feature = "num-rational")] N :: r8 ($ v) => $ arm , # [cfg (feature = "num-rational")] N :: r16 ($ v) => $ arm , # [cfg (feature = "num-rational")] N :: r32 ($ v) => $ arm , # [cfg (feature = "num-rational")] N :: r64 ($ v) => $ arm , # [cfg (feature = "num-rational")] N :: r128 ($ v) => $ arm , # [cfg (feature = "num-rational")] N :: rsize ($ v) => $ arm , # [cfg (all (feature = "num-bigint" , feature = "num-rational"))] N :: rbig ($ v) => $ arm , # [cfg (feature = "num-complex")] N :: c32 ($ v) => $ arm , # [cfg (feature = "num-complex")] N :: c64 ($ v) => $ arm , } } ; }
};
}
