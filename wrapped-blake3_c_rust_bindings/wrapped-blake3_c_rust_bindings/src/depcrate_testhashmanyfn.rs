// Generated macro for HashManyFn (type)
macro_rules! Depcrate_testHashManyFn {
() => {
// Module: crate::test
// Provides: {"HashManyFn"}
// Dependencies: {}
type HashManyFn = unsafe extern "C" fn (inputs : * const * const u8 , num_inputs : usize , blocks : usize , key : * const u32 , counter : u64 , increment_counter : bool , flags : u8 , flags_start : u8 , flags_end : u8 , out : * mut u8 ,) ;
};
}
