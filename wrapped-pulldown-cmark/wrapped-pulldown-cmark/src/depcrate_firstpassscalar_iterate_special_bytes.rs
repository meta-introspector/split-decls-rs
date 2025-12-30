// Generated macro for scalar_iterate_special_bytes (function)
macro_rules! Depcrate_firstpassscalar_iterate_special_bytes {
() => {
// Module: crate::firstpass
// Provides: {"scalar_iterate_special_bytes"}
// Dependencies: {}
fn scalar_iterate_special_bytes < F , T > (lut : & [bool ; 256] , bytes : & [u8] , mut ix : usize , mut callback : F ,) -> (usize , Option < T >) where F : FnMut (usize , u8) -> LoopInstruction < Option < T > > , { while ix < bytes . len () { let b = bytes [ix] ; if lut [b as usize] { match callback (ix , b) { LoopInstruction :: ContinueAndSkip (skip) => { ix += skip ; } LoopInstruction :: BreakAtWith (ix , val) => { return (ix , val) ; } } } ix += 1 ; } (ix , None) }
};
}
