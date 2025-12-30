// Generated macro for reject_rand (function)
macro_rules! Depcrate_base_helperreject_rand {
() => {
// Module: crate::base::helper
// Provides: {"reject_rand"}
// Dependencies: {}
# [doc (hidden)] # [inline] # [cfg (feature = "rand-no-std")] pub fn reject_rand < G : Rng + ? Sized , F : FnMut (& T) -> bool , T > (g : & mut G , f : F) -> T where Standard : Distribution < T > , { use std :: iter ; iter :: repeat (()) . map (| _ | g . gen ()) . find (f) . unwrap () }
};
}
