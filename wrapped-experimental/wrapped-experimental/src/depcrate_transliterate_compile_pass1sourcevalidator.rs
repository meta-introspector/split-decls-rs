// Generated macro for SourceValidator (struct)
macro_rules! Depcrate_transliterate_compile_pass1SourceValidator {
() => {
// Module: crate::transliterate::compile::pass1
// Provides: {"SourceValidator"}
// Dependencies: {}
struct SourceValidator < 'a , 'p , F : Fn (& str) -> bool > { is_variable_defined : F , data : & 'a mut Pass1Data , ante : & 'p [parse :: Element] , key : & 'p [parse :: Element] , post : & 'p [parse :: Element] , num_segments : u32 , }
};
}
