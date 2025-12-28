macro_rules! add_version_to_llvm_target {
    () => {
        pub (super) fn add_version_to_llvm_target (llvm_target : & str , deployment_target : OSVersion ,) -> String { let mut components = llvm_target . split ("-") ; let arch = components . next () . expect ("apple target should have arch") ; let vendor = components . next () . expect ("apple target should have vendor") ; let os = components . next () . expect ("apple target should have os") ; let environment = components . next () ; assert_eq ! (components . next () , None , "too many LLVM triple components") ; assert ! (! os . contains (| c : char | c . is_ascii_digit ()) , "LLVM target must not already be versioned") ; let version = deployment_target . fmt_full () ; if let Some (env) = environment { format ! ("{arch}-{vendor}-{os}{version}-{env}") } else { format ! ("{arch}-{vendor}-{os}{version}") } }
    };
}

add_version_to_llvm_target!();