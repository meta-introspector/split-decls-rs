// Generated macro for ensure_read (function)
macro_rules! Depcrate_reprensure_read {
() => {
// Module: crate::repr
// Provides: {"ensure_read"}
// Dependencies: {}
# [doc = " Returns the supplied value, and ensures that the value is eagerly loaded into a register."] # [inline (always)] fn ensure_read (value : usize) -> usize { # [cfg (all (not (miri) , any (target_arch = "x86" , target_arch = "x86_64" , target_arch = "arm" , target_arch = "aarch64" ,)))] unsafe { core :: arch :: asm ! ("/* {value} */" , value = in (reg) value , options (nomem , nostack) ,) ; } ; value }
};
}
