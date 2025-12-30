// Generated macro for capture_output_example (function)
macro_rules! Depcratecapture_output_example {
() => {
// Module: crate
// Provides: {"capture_output_example"}
// Dependencies: {}
fn capture_output_example () { let builder = PersistingHasherBuilder :: default () ; let mut map = HashMap :: with_capacity_and_hasher (10 , builder) ; map . insert (1 , 2) ; map . insert (3 , 4) ; let builder = PersistingHasherBuilder :: default () ; let mut map = HashMap :: with_capacity_and_hasher (10 , builder) ; map . insert ("1" , 2) ; map . insert ("3" , 4) ; PersistingHasherBuilder :: default () . flush () ; }
};
}
