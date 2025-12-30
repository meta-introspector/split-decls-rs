// Generated macro for SmartSubtransportStream (trait)
macro_rules! Depcrate_transportSmartSubtransportStream {
() => {
// Module: crate::transport
// Provides: {"SmartSubtransportStream"}
// Dependencies: {}
# [doc = " An instance of a stream over which a smart transport will communicate with a"] # [doc = " remote."] # [doc = ""] # [doc = " Currently this only requires the standard `Read` and `Write` traits. This"] # [doc = " trait also does not need to be implemented manually as long as the `Read`"] # [doc = " and `Write` traits are implemented."] pub trait SmartSubtransportStream : Read + Write + Send + 'static { }
};
}
