// Generated macro for defraw (macro)
macro_rules! Depcrate_arch_aarch64_memchrdefraw {
() => {
// Module: crate::arch::aarch64::memchr
// Provides: {"defraw"}
// Dependencies: {}
macro_rules ! defraw { ($ ty : ident , $ find : ident , $ start : ident , $ end : ident , $ ($ needles : ident) ,+) => { { # [cfg (target_feature = "neon")] { use crate :: arch :: aarch64 :: neon :: memchr ::$ ty ; debug ! ("chose neon for {}" , stringify ! ($ ty)) ; debug_assert ! ($ ty :: is_available ()) ; $ ty :: new_unchecked ($ ($ needles) ,+) .$ find ($ start , $ end) } # [cfg (not (target_feature = "neon"))] { use crate :: arch :: all :: memchr ::$ ty ; debug ! ("no neon feature available, using fallback for {}" , stringify ! ($ ty) ,) ; $ ty :: new ($ ($ needles) ,+) .$ find ($ start , $ end) } } } }
};
}
