// Generated macro for SharedLibrary (struct)
macro_rules! Depcrate_linuxSharedLibrary {
() => {
// Module: crate::linux
// Provides: {"SharedLibrary"}
// Dependencies: {}
# [doc = " A shared library on Linux."] pub struct SharedLibrary < 'a > { size : usize , addr : * const u8 , name : Cow < 'a , CStr > , headers : & 'a [Phdr] , }
};
}
