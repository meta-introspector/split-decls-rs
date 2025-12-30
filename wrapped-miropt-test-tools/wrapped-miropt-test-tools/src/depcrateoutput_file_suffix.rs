// Generated macro for output_file_suffix (function)
macro_rules! Depcrateoutput_file_suffix {
() => {
// Module: crate
// Provides: {"output_file_suffix"}
// Dependencies: {}
fn output_file_suffix (testfile : & Path , bit_width : u32 , panic_strategy : PanicStrategy) -> String { let mut each_bit_width = false ; let mut each_panic_strategy = false ; for line in fs :: read_to_string (testfile) . unwrap () . lines () { if line == "// EMIT_MIR_FOR_EACH_BIT_WIDTH" { each_bit_width = true ; } if line == "// EMIT_MIR_FOR_EACH_PANIC_STRATEGY" { each_panic_strategy = true ; } } let mut suffix = String :: new () ; if each_bit_width { suffix . push_str (& format ! (".{bit_width}bit")) ; } if each_panic_strategy { match panic_strategy { PanicStrategy :: Unwind => suffix . push_str (".panic-unwind") , PanicStrategy :: Abort => suffix . push_str (".panic-abort") , } } suffix }
};
}
