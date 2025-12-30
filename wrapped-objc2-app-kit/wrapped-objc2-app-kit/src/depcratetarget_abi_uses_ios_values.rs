// Generated macro for TARGET_ABI_USES_IOS_VALUES (const)
macro_rules! DepcrateTARGET_ABI_USES_IOS_VALUES {
() => {
// Module: crate
// Provides: {"TARGET_ABI_USES_IOS_VALUES"}
// Dependencies: {}
# [doc = " (!TARGET_CPU_X86_64 || (TARGET_OS_IPHONE && !TARGET_OS_MACCATALYST))"] # [doc = ""] # [doc = " <https://github.com/xamarin/xamarin-macios/issues/12111>"] # [allow (unused)] # [allow (unexpected_cfgs)] pub (crate) const TARGET_ABI_USES_IOS_VALUES : bool = ! cfg ! (target_arch = "x86_64") || (cfg ! (all (target_vendor = "apple" , not (target_os = "macos"))) && ! cfg ! (target_env = "macabi")) ;
};
}
