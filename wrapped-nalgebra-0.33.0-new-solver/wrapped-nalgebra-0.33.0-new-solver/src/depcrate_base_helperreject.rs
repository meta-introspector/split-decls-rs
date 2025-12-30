// Generated macro for reject (function)
macro_rules! Depcrate_base_helperreject {
() => {
// Module: crate::base::helper
// Provides: {"reject"}
// Dependencies: {}
# [doc = " Simple helper function for rejection sampling"] # [cfg (feature = "arbitrary")] # [doc (hidden)] # [inline] pub fn reject < F : FnMut (& T) -> bool , T : Arbitrary > (g : & mut Gen , f : F) -> T { use std :: iter ; iter :: repeat (()) . map (| _ | Arbitrary :: arbitrary (g)) . find (f) . unwrap () }
};
}
