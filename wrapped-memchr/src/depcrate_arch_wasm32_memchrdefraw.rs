// Generated macro for defraw (macro)
macro_rules! Depcrate_arch_wasm32_memchrdefraw {
() => {
// Module: crate::arch::wasm32::memchr
// Provides: {"defraw"}
// Dependencies: {}
macro_rules ! defraw { ($ ty : ident , $ find : ident , $ start : ident , $ end : ident , $ ($ needles : ident) ,+) => { { use crate :: arch :: wasm32 :: simd128 :: memchr ::$ ty ; debug ! ("chose simd128 for {}" , stringify ! ($ ty)) ; debug_assert ! ($ ty :: is_available ()) ; $ ty :: new_unchecked ($ ($ needles) ,+) .$ find ($ start , $ end) } } }
};
}
