// Generated macro for impl_683 (impl)
macro_rules! Depcrate_imp_float_aarch64impl_683 {
() => {
// Module: crate::imp::float::aarch64
// Provides: {"impl_683"}
// Dependencies: {}
# [cfg (portable_atomic_unstable_f128)] impl AtomicF128 { # [inline] pub (crate) fn fetch_add (& self , val : f128 , order : Ordering) -> f128 { self . fetch_update_ (order , | x | x + val) } # [inline] pub (crate) fn fetch_sub (& self , val : f128 , order : Ordering) -> f128 { self . fetch_update_ (order , | x | x - val) } # [inline] pub (super) fn fetch_update_ < F > (& self , order : Ordering , mut f : F) -> f128 where F : FnMut (f128) -> f128 , { let mut prev = self . load (Ordering :: Relaxed) ; loop { let next = f (prev) ; match self . compare_exchange_weak (prev , next , order , Ordering :: Relaxed) { Ok (x) => return x , Err (next_prev) => prev = next_prev , } } } # [inline] pub (crate) fn fetch_max (& self , val : f128 , order : Ordering) -> f128 { self . fetch_update_ (order , | x | x . max (val)) } # [inline] pub (crate) fn fetch_min (& self , val : f128 , order : Ordering) -> f128 { self . fetch_update_ (order , | x | x . min (val)) } }
};
}
