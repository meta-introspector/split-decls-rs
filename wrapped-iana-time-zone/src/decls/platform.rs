macro_rules! platform {
    () => {
        # [cfg_attr (any (all (target_os = "linux" , not (target_env = "ohos")) , target_os = "hurd") , path = "tz_linux.rs")] # [cfg_attr (all (target_os = "linux" , target_env = "ohos") , path = "tz_ohos.rs")] # [cfg_attr (target_os = "windows" , path = "tz_windows.rs")] # [cfg_attr (target_vendor = "apple" , path = "tz_darwin.rs")] # [cfg_attr (all (target_arch = "wasm32" , target_os = "unknown") , path = "tz_wasm32_unknown.rs")] # [cfg_attr (any (target_os = "freebsd" , target_os = "dragonfly") , path = "tz_freebsd.rs")] # [cfg_attr (any (target_os = "netbsd" , target_os = "openbsd") , path = "tz_netbsd.rs")] # [cfg_attr (any (target_os = "illumos" , target_os = "solaris") , path = "tz_illumos.rs")] # [cfg_attr (target_os = "aix" , path = "tz_aix.rs")] # [cfg_attr (target_os = "android" , path = "tz_android.rs")] # [cfg_attr (target_os = "haiku" , path = "tz_haiku.rs")] mod platform ;
    };
}

platform!()