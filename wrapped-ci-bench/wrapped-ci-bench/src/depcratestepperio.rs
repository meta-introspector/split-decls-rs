// Generated macro for StepperIo (struct)
macro_rules! DepcrateStepperIo {
() => {
// Module: crate
// Provides: {"StepperIo"}
// Dependencies: {}
# [doc = " Stepper fields necessary for IO"] struct StepperIo < 'a > { reader : & 'a mut dyn AsyncRead , writer : & 'a mut dyn AsyncWrite , handshake_buf : & 'a mut [u8] , }
};
}
