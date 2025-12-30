// Generated macro for PinState (enum)
macro_rules! Depcrate_digitalPinState {
() => {
// Module: crate::digital
// Provides: {"PinState"}
// Dependencies: {}
# [doc = " Digital output pin state."] # [doc = ""] # [doc = " Conversion from `bool` and logical negation are also implemented"] # [doc = " for this type."] # [doc = " ```rust"] # [doc = " # use embedded_hal::digital::PinState;"] # [doc = " let state = PinState::from(false);"] # [doc = " assert_eq!(state, PinState::Low);"] # [doc = " assert_eq!(!state, PinState::High);"] # [doc = " ```"] # [derive (Debug , PartialEq , Eq , Clone , Copy)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum PinState { # [doc = " Low pin state."] Low , # [doc = " High pin state."] High , }
};
}
