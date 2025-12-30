// Generated macro for Feature (struct)
macro_rules! DepcrateFeature {
() => {
// Module: crate
// Provides: {"Feature"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct Feature { pub name : Symbol , # [doc = " For unstable features: the version the feature was added in."] # [doc = " For accepted features: the version the feature got stabilized in."] # [doc = " For removed features we are inconsistent; sometimes this is the"] # [doc = " version it got added, sometimes the version it got removed."] pub since : & 'static str , issue : Option < NonZero < u32 > > , }
};
}
