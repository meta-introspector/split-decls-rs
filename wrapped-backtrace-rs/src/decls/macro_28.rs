macro_rules! macro_28 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (miri)] { mod miri ; use miri as imp ; } else if # [cfg (all (windows , target_env = "msvc" , not (target_vendor = "uwp")))] { mod dbghelp ; use dbghelp as imp ; } else if # [cfg (all (any (unix , all (windows , target_env = "gnu")) , not (target_vendor = "uwp") , not (target_os = "emscripten") , any (not (backtrace_in_libstd) , feature = "backtrace") ,))] { mod gimli ; use gimli as imp ; } else { mod noop ; use noop as imp ; } }
    };
}

macro_28!();