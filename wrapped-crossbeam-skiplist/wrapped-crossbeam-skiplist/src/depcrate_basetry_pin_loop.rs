// Generated macro for try_pin_loop (function)
macro_rules! Depcrate_basetry_pin_loop {
() => {
// Module: crate::base
// Provides: {"try_pin_loop"}
// Dependencies: {}
# [doc = " Helper function to retry an operation until pinning succeeds or `None` is"] # [doc = " returned."] pub (crate) fn try_pin_loop < 'a : 'g , 'g , F , K , V > (mut f : F) -> Option < RefEntry < 'a , K , V > > where F : FnMut () -> Option < Entry < 'a , 'g , K , V > > , { loop { if let Some (e) = f () ? . pin () { return Some (e) ; } } }
};
}
