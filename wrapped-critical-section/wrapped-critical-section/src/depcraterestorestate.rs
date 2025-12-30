// Generated macro for RestoreState (struct)
macro_rules! DepcrateRestoreState {
() => {
// Module: crate
// Provides: {"RestoreState"}
// Dependencies: {}
# [doc = " Opaque \"restore state\"."] # [doc = ""] # [doc = " Implementations use this to \"carry over\" information between acquiring and releasing"] # [doc = " a critical section. For example, when nesting two critical sections of an"] # [doc = " implementation that disables interrupts globally, acquiring the inner one won't disable"] # [doc = " the interrupts since they're already disabled. The impl would use the restore state to \"tell\""] # [doc = " the corresponding release that it does *not* have to reenable interrupts yet, only the"] # [doc = " outer release should do so."] # [doc = ""] # [doc = " User code uses [`RestoreState`] opaquely, critical section implementations"] # [doc = " use [`RawRestoreState`] so that they can use the inner value."] # [derive (Clone , Copy , Debug)] pub struct RestoreState (RawRestoreState) ;
};
}
