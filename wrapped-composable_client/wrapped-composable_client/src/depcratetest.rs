// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [test] fn test () -> Result < () > { unsafe { interop () . ok () ? } ; let compositor = Compositor :: new () ? ; let container = compositor . CreateContainerVisual (123) ? ; assert_eq ! (container . Children () , 123) ; assert_eq ! (container . Compositor () ?, compositor) ; let sprite = compositor . CreateSpriteVisual (456) ? ; assert_eq ! (sprite . Brush () , 456) ; assert_eq ! (sprite . Children () , 456 * 2) ; assert_eq ! (sprite . Compositor () ?, compositor) ; Ok (()) }
};
}
