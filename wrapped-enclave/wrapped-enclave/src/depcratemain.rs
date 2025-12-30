// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let value : u32 = 41 ; let question = "Answer to the Ultimate Question of Life, the Universe, and Everything:" ; unsafe { println ! ("{}: {}!" , question , rust_plus_one_global_asm (& value)) ; println ! ("{}: {}!" , question , cc_plus_one_c (& value)) ; println ! ("{}: {}!" , question , cc_plus_one_c_asm (& value)) ; println ! ("{}: {}!" , question , cc_plus_one_cxx (& value)) ; println ! ("{}: {}!" , question , cc_plus_one_cxx_asm (& value)) ; println ! ("{}: {}!" , question , cc_plus_one_asm (& value)) ; println ! ("{}: {}!" , question , cmake_plus_one_c (& value)) ; println ! ("{}: {}!" , question , cmake_plus_one_c_asm (& value)) ; println ! ("{}: {}!" , question , cmake_plus_one_cxx (& value)) ; println ! ("{}: {}!" , question , cmake_plus_one_cxx_asm (& value)) ; println ! ("{}: {}!" , question , cmake_plus_one_c_global_asm (& value)) ; println ! ("{}: {}!" , question , cmake_plus_one_cxx_global_asm (& value)) ; println ! ("{}: {}!" , question , cmake_plus_one_asm (& value)) ; } }
};
}
