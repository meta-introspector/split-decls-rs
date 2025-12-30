// Generated macro for CowBytes (struct)
macro_rules! Depcrate_cowCowBytes {
() => {
// Module: crate::cow
// Provides: {"CowBytes"}
// Dependencies: {}
# [doc = " A specialized copy-on-write byte string."] # [doc = ""] # [doc = " The purpose of this type is to permit usage of a \"borrowed or owned"] # [doc = " byte string\" in a way that keeps std/no-std compatibility. That is, in"] # [doc = " no-std/alloc mode, this type devolves into a simple &[u8] with no owned"] # [doc = " variant available. We can't just use a plain Cow because Cow is not in"] # [doc = " core."] # [derive (Clone , Debug)] pub struct CowBytes < 'a > (Imp < 'a >) ;
};
}
