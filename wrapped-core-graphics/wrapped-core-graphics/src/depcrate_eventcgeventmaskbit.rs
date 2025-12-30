// Generated macro for CGEventMaskBit (macro)
macro_rules! Depcrate_eventCGEventMaskBit {
() => {
// Module: crate::event
// Provides: {"CGEventMaskBit"}
// Dependencies: {}
macro_rules ! CGEventMaskBit { ($ eventType : expr) => { (1 << $ eventType as CGEventMask) } ; }
};
}
