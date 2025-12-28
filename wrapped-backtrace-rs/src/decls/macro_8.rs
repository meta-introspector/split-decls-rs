macro_rules! deps {
    () => {
        Frame!();
    };
}

macro_rules! macro_8 {
    () => {
        deps!();
        cfg_if :: cfg_if ! { if # [cfg (miri)] { pub (crate) mod miri ; use self :: miri :: trace as trace_imp ; pub (crate) use self :: miri :: Frame as FrameImp ; } else if # [cfg (any (all (unix , not (target_os = "emscripten") , not (all (target_os = "ios" , target_arch = "arm")) ,) , all (target_env = "sgx" , target_vendor = "fortanix" ,) ,))] { mod libunwind ; use self :: libunwind :: trace as trace_imp ; pub (crate) use self :: libunwind :: Frame as FrameImp ; } else if # [cfg (all (windows , not (target_vendor = "uwp")))] { cfg_if :: cfg_if ! { if # [cfg (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "arm64ec"))] { mod win64 ; use self :: win64 :: trace as trace_imp ; pub (crate) use self :: win64 :: Frame as FrameImp ; } else if # [cfg (any (target_arch = "x86" , target_arch = "arm"))] { mod win32 ; use self :: win32 :: trace as trace_imp ; pub (crate) use self :: win32 :: Frame as FrameImp ; } } } else { mod noop ; use self :: noop :: trace as trace_imp ; pub (crate) use self :: noop :: Frame as FrameImp ; } }
    };
}

macro_8!()