// Generated macro for qmap (function)
macro_rules! Depcrate_genericqmap {
() => {
// Module: crate::generic
// Provides: {"qmap"}
// Dependencies: {}
# [inline (always)] fn qmap < T , F > (t : T , f : F) -> T where T : Store < vec128_storage > + Into < vec128_storage > , F : Fn (u64) -> u64 , { let t : vec128_storage = t . into () ; let q = unsafe { t . q } ; let q = vec128_storage { q : [f (q [0]) , f (q [1])] , } ; unsafe { T :: unpack (q) } }
};
}
