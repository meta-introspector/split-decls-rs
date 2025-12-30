// Generated macro for State (enum)
macro_rules! Depcrate_encoderState {
() => {
// Module: crate::encoder
// Provides: {"State"}
// Dependencies: {}
# [doc = " Current state of the encoder."] # [derive (Debug)] enum State { # [doc = " Initial state - no arcs yet encoded."] Initial , # [doc = " First arc has been supplied and stored as the wrapped [`Arc`]."] FirstArc (Arc) , # [doc = " Encoding base 128 body of the OID."] Body , }
};
}
