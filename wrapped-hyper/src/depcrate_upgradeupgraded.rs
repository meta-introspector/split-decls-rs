// Generated macro for Upgraded (struct)
macro_rules! Depcrate_upgradeUpgraded {
() => {
// Module: crate::upgrade
// Provides: {"Upgraded"}
// Dependencies: {}
# [doc = " An upgraded HTTP connection."] # [doc = ""] # [doc = " This type holds a trait object internally of the original IO that"] # [doc = " was used to speak HTTP before the upgrade. It can be used directly"] # [doc = " as a [`Read`] or [`Write`] for convenience."] # [doc = ""] # [doc = " Alternatively, if the exact type is known, this can be deconstructed"] # [doc = " into its parts."] pub struct Upgraded { io : Rewind < Box < dyn Io + Send > > , }
};
}
