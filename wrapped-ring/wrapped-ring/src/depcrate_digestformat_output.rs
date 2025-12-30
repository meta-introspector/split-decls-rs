// Generated macro for format_output (function)
macro_rules! Depcrate_digestformat_output {
() => {
// Module: crate::digest
// Provides: {"format_output"}
// Dependencies: {}
# [inline] fn format_output < T , F , const N : usize > (input : [Wrapping < T > ; sha2 :: CHAINING_WORDS] , f : F) -> Output where F : Fn (T) -> [u8 ; N] , T : Copy , { let mut output = Output ([0 ; MAX_OUTPUT_LEN]) ; output . 0 . chunks_mut (N) . zip (input . iter () . copied () . map (| Wrapping (w) | f (w))) . for_each (| (o , i) | { o . copy_from_slice (& i) ; }) ; output }
};
}
