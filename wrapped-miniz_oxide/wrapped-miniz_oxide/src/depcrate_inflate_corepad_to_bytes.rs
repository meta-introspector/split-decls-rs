// Generated macro for pad_to_bytes (function)
macro_rules! Depcrate_inflate_corepad_to_bytes {
() => {
// Module: crate::inflate::core
// Provides: {"pad_to_bytes"}
// Dependencies: {}
# [inline] fn pad_to_bytes < F > (l : & mut LocalVars , in_iter : & mut InputWrapper , flags : u32 , f : F) -> Action where F : FnOnce (& mut LocalVars) -> Action , { let num_bits = l . num_bits & 7 ; read_bits (l , num_bits , in_iter , flags , | l , _ | f (l)) }
};
}
