// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl Dit { # [doc = " Initialize Data-Independent Timing using runtime CPU feature detection."] pub fn init () -> Self { Self { supported : dit_supported :: init () , } } # [doc = " Enable Data-Independent Timing (if available)."] # [doc = ""] # [doc = " Returns an RAII guard that will return DIT to its previous state when dropped."] # [must_use] pub fn enable (& self) -> Guard < '_ > { let was_enabled = if self . is_supported () { unsafe { set_dit_enabled () } } else { false } ; Guard { dit : self , was_enabled , } } # [doc = " Check if DIT has been enabled."] pub fn is_enabled (& self) -> bool { if self . is_supported () { unsafe { get_dit_enabled () } } else { false } } # [doc = " Check if DIT is supported by this CPU."] pub fn is_supported (& self) -> bool { self . supported . get () } }
};
}
