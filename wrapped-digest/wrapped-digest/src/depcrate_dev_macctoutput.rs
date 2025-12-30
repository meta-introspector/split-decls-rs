// Generated macro for CtOutput (struct)
macro_rules! Depcrate_dev_macCtOutput {
() => {
// Module: crate::dev::mac
// Provides: {"CtOutput"}
// Dependencies: {}
# [doc = " Fixed size output value which provides a safe [`Eq`] implementation that"] # [doc = " runs in constant time."] # [doc = ""] # [doc = " It is useful for implementing Message Authentication Codes (MACs)."] # [derive (Clone)] pub struct CtOutput < T : OutputSizeUser > { bytes : Output < T > , }
};
}
