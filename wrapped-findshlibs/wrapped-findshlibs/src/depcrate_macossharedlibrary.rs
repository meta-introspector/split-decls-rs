// Generated macro for SharedLibrary (struct)
macro_rules! Depcrate_macosSharedLibrary {
() => {
// Module: crate::macos
// Provides: {"SharedLibrary"}
// Dependencies: {}
# [doc = " The MacOS implementation of the [SharedLibrary"] # [doc = " trait](../trait.SharedLibrary.html)."] # [doc = ""] # [doc = " This wraps the `_dyld_image_count` and"] # [doc = " `_dyld_get_image_{header,vmaddr_slide,name}` system APIs from the"] # [doc = " `<mach-o/dyld.h>` header."] pub struct SharedLibrary < 'a > { header : MachHeader < 'a > , slide : usize , name : & 'a CStr , }
};
}
