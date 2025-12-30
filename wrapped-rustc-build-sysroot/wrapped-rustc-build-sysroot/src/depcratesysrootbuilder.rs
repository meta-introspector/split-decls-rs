// Generated macro for SysrootBuilder (struct)
macro_rules! DepcrateSysrootBuilder {
() => {
// Module: crate
// Provides: {"SysrootBuilder"}
// Dependencies: {}
# [doc = " Information about a to-be-created sysroot."] pub struct SysrootBuilder < 'a > { sysroot_dir : PathBuf , target : OsString , config : SysrootConfig , mode : BuildMode , rustflags : Vec < OsString > , cargo : Option < Command > , rustc_version : Option < rustc_version :: VersionMeta > , when_build_required : Option < Box < dyn FnOnce () + 'a > > , }
};
}
