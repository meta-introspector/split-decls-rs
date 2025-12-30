// Generated macro for EncodeRequest (macro)
macro_rules! Depcrate_macrosEncodeRequest {
() => {
// Module: crate::macros
// Provides: {"EncodeRequest"}
// Dependencies: {}
macro_rules ! EncodeRequest { ($ request : expr , $ direction : expr , $ type : expr , $ recipient : expr) => { (($ request << 8) + ($ recipient + ($ type << kUSBRqTypeShift) + ($ direction << kUSBRqDirnShift))) } ; }
};
}
