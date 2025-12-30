// Generated macro for impl_28 (impl)
macro_rules! Depcrate_readerimpl_28 {
() => {
// Module: crate::reader
// Provides: {"impl_28"}
// Dependencies: {}
impl DfaClasses { fn new () -> DfaClasses { DfaClasses { classes : [0 ; CLASS_SIZE] , next_class : 1 } } fn add (& mut self , b : u8) { if self . next_class > CLASS_SIZE { panic ! ("added too many classes") } self . classes [b as usize] = self . next_class as u8 ; self . next_class += 1 ; } fn num_classes (& self) -> usize { self . next_class } # [doc = " Scan and copy the input bytes to the output buffer quickly."] # [doc = ""] # [doc = " This assumes that the current state of the DFA is either `InField` or"] # [doc = " `InQuotedField`. In this case, all bytes corresponding to the first"] # [doc = " equivalence class (i.e., not a delimiter/quote/escape/etc.) are"] # [doc = " guaranteed to never result in a state transition out of the current"] # [doc = " state. This function takes advantage of that copies every byte from"] # [doc = " `input` in the first equivalence class to `output`. Once a byte is seen"] # [doc = " outside the first equivalence class, we quit and should fall back to"] # [doc = " the main DFA loop."] # [inline (always)] fn scan_and_copy (& self , input : & [u8] , nin : & mut usize , output : & mut [u8] , nout : & mut usize ,) { while * nin < input . len () && * nout < output . len () && self . classes [input [* nin] as usize] == 0 { output [* nout] = input [* nin] ; * nin += 1 ; * nout += 1 ; } } }
};
}
