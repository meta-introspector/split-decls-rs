macro_rules! versioned_llvm_target {
    () => {
        # [doc = " The target triple depends on the deployment target, and is required to"] # [doc = " enable features such as cross-language LTO, and for picking the right"] # [doc = " Mach-O commands."] # [doc = ""] # [doc = " Certain optimizations also depend on the deployment target."] pub fn versioned_llvm_target (sess : & Session) -> Cow < '_ , str > { if sess . target . is_like_darwin { apple :: add_version_to_llvm_target (& sess . target . llvm_target , sess . apple_deployment_target ()) . into () } else { Cow :: Borrowed (& sess . target . llvm_target) } }
    };
}

versioned_llvm_target!();