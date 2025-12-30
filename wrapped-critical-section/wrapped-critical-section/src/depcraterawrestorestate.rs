// Generated macro for RawRestoreState (type)
macro_rules! DepcrateRawRestoreState {
() => {
// Module: crate
// Provides: {"RawRestoreState"}
// Dependencies: {}
# [doc = " Raw, transparent \"restore state\"."] # [doc = ""] # [doc = " This type changes based on which Cargo feature is selected, out of"] # [doc = " - `restore-state-none` (default, makes the type be `()`)"] # [doc = " - `restore-state-bool`"] # [doc = " - `restore-state-u8`"] # [doc = " - `restore-state-u16`"] # [doc = " - `restore-state-u32`"] # [doc = " - `restore-state-u64`"] # [doc = " - `restore-state-usize`"] # [doc = ""] # [doc = " See [`RestoreState`]."] # [doc = ""] # [doc = " User code uses [`RestoreState`] opaquely, critical section implementations"] # [doc = " use [`RawRestoreState`] so that they can use the inner value."] pub type RawRestoreState = RawRestoreStateInner ;
};
}
