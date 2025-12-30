// Generated macro for write_main_cpp (function)
macro_rules! Depcrate_common_gen_cwrite_main_cpp {
() => {
// Module: crate::common::gen_c
// Provides: {"write_main_cpp"}
// Dependencies: {}
pub fn write_main_cpp < 'a > (w : & mut impl std :: io :: Write , arch_specific_definitions : & str , arch_specific_headers : & [& str] , intrinsics : impl Iterator < Item = & 'a str > + Clone ,) -> std :: io :: Result < () > { for header in COMMON_HEADERS . iter () . chain (arch_specific_headers . iter ()) { writeln ! (w , "#include <{header}>") ? ; } writeln ! (w , "{arch_specific_definitions }") ? ; for intrinsic in intrinsics . clone () { writeln ! (w , "extern int run_{intrinsic}(void);") ? ; } writeln ! (w , "int main(int argc, char **argv) {{") ? ; for intrinsic in intrinsics { writeln ! (w , "    std::cout << \"{INTRINSIC_DELIMITER}\" << std::endl;") ? ; writeln ! (w , "    std::cout << \"{intrinsic}\" << std::endl;") ? ; writeln ! (w , "    run_{intrinsic}();\n") ? ; } writeln ! (w , "    return 0;") ? ; writeln ! (w , "}}") ? ; Ok (()) }
};
}
