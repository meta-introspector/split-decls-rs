// Generated macro for CaptureKind (enum)
macro_rules! DepcrateCaptureKind {
() => {
// Module: crate
// Provides: {"CaptureKind"}
// Dependencies: {}
# [doc = " How a local is captured by a closure"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum CaptureKind { Value , Use , Ref (Mutability) , }
};
}
