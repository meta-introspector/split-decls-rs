// Generated macro for BOOT_ROOT_PAGE_TABLE (static)
macro_rules! Depcrate_schedulerBOOT_ROOT_PAGE_TABLE {
() => {
// Module: crate::scheduler
// Provides: {"BOOT_ROOT_PAGE_TABLE"}
// Dependencies: {}
# [cfg (all (target_arch = "x86_64" , feature = "common-os"))] pub (crate) static BOOT_ROOT_PAGE_TABLE : OnceCell < usize > = OnceCell :: new () ;
};
}
