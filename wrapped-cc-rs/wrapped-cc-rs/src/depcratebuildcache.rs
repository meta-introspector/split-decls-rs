// Generated macro for BuildCache (struct)
macro_rules! DepcrateBuildCache {
() => {
// Module: crate
// Provides: {"BuildCache"}
// Dependencies: {}
# [derive (Debug , Default)] struct BuildCache { env_cache : RwLock < HashMap < Box < str > , Env > > , apple_sdk_root_cache : RwLock < HashMap < Box < str > , Arc < OsStr > > > , apple_versions_cache : RwLock < HashMap < Box < str > , Arc < str > > > , cached_compiler_family : RwLock < CompilerFamilyLookupCache > , known_flag_support_status_cache : RwLock < HashMap < CompilerFlag , bool > > , target_info_parser : target :: TargetInfoParser , }
};
}
