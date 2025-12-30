// Generated macro for write_main_rs (function)
macro_rules! Depcrate_common_gen_rustwrite_main_rs {
() => {
// Module: crate::common::gen_rust
// Provides: {"write_main_rs"}
// Dependencies: {}
pub fn write_main_rs < 'a > (w : & mut impl std :: io :: Write , chunk_count : usize , cfg : & str , definitions : & str , intrinsics : impl Iterator < Item = & 'a str > + Clone ,) -> std :: io :: Result < () > { writeln ! (w , "#![feature(simd_ffi)]") ? ; writeln ! (w , "#![feature(f16)]") ? ; writeln ! (w , "#![allow(unused)]") ? ; writeln ! (w , "#![allow(non_upper_case_globals)]") ? ; writeln ! (w , "#![allow(non_camel_case_types)]") ? ; writeln ! (w , "#![allow(non_snake_case)]") ? ; writeln ! (w , "{cfg}") ? ; writeln ! (w , "{definitions}") ? ; for module in 0 .. chunk_count { writeln ! (w , "use mod_{module}::*;") ? ; } writeln ! (w , "fn main() {{") ? ; for binary in intrinsics { writeln ! (w , "    println!(\"{INTRINSIC_DELIMITER}\");") ? ; writeln ! (w , "    println!(\"{binary}\");") ? ; writeln ! (w , "    run_{binary}();\n") ? ; } writeln ! (w , "}}") ? ; Ok (()) }
};
}
