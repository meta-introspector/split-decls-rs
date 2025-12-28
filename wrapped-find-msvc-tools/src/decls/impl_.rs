macro_rules! deps {
    () => {
        Tool!();
        EnvGetter!();
        TargetArch!();
    };
}

macro_rules! impl_ {
    () => {
        deps!();
        # [doc = " Non-Windows Implementation."] # [cfg (not (windows))] mod impl_ { use std :: { env , ffi :: OsStr , path :: PathBuf } ; use super :: { EnvGetter , TargetArch } ; use crate :: Tool ; # [doc = " Finding msbuild.exe tool under unix system is not currently supported."] # [doc = " Maybe can check it using an environment variable looks like `MSBUILD_BIN`."] # [inline (always)] pub (super) fn find_msbuild (_target : TargetArch , _ : & dyn EnvGetter) -> Option < Tool > { None } # [inline (always)] pub (super) fn find_devenv (_target : TargetArch , _ : & dyn EnvGetter) -> Option < Tool > { None } # [inline (always)] pub (super) fn find_llvm_tool (_tool : & str , _target : TargetArch , _ : & dyn EnvGetter ,) -> Option < Tool > { None } # [doc = " Attempt to find the tool using environment variables set by vcvars."] pub (super) fn find_msvc_environment (tool : & str , _target : TargetArch , env_getter : & dyn EnvGetter ,) -> Option < Tool > { let vc_install_dir = env_getter . get_env ("VCINSTALLDIR") ? ; let vs_install_dir = env_getter . get_env ("VSINSTALLDIR") ? ; let get_tool = | install_dir : & OsStr | { env :: split_paths (install_dir) . map (| p | p . join (tool)) . find (| p | p . exists ()) . map (| path | Tool { tool : path , is_clang_cl : false , env : Vec :: new () , }) } ; get_tool (vc_install_dir . as_ref ()) . or_else (| | get_tool (vs_install_dir . as_ref ())) . or_else (| | { env_getter . get_env ("PATH") . as_ref () . map (| path | path . as_ref ()) . and_then (get_tool) }) } # [inline (always)] pub (super) fn find_msvc_15plus (_tool : & str , _target : TargetArch , _ : & dyn EnvGetter ,) -> Option < Tool > { None } # [inline (always)] pub (super) fn find_msvc_14 (_tool : & str , _target : TargetArch , _ : & dyn EnvGetter ,) -> Option < Tool > { None } # [inline (always)] pub (super) fn has_msbuild_version (_version : & str , _ : & dyn EnvGetter) -> bool { false } # [inline (always)] pub (super) fn get_ucrt_dir () -> Option < (PathBuf , String) > { None } }
    };
}

impl_!()