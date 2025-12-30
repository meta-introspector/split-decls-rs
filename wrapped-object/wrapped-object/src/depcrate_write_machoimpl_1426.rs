// Generated macro for impl_1426 (impl)
macro_rules! Depcrate_write_machoimpl_1426 {
() => {
// Module: crate::write::macho
// Provides: {"impl_1426"}
// Dependencies: {}
impl < 'a > Object < 'a > { # [doc = " Specify the Mach-O CPU subtype."] # [doc = ""] # [doc = " Requires `feature = \"macho\"`."] # [inline] pub fn set_macho_cpu_subtype (& mut self , cpu_subtype : u32) { self . macho_cpu_subtype = Some (cpu_subtype) ; } # [doc = " Specify information for a Mach-O `LC_BUILD_VERSION` command."] # [doc = ""] # [doc = " Requires `feature = \"macho\"`."] # [inline] pub fn set_macho_build_version (& mut self , info : MachOBuildVersion) { self . macho_build_version = Some (info) ; } }
};
}
