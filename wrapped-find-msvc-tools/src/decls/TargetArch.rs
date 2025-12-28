macro_rules! TargetArch {
    () => {
        # [doc = " The target provided by the user."] # [derive (Copy , Clone , PartialEq , Eq)] enum TargetArch { X86 , X64 , Arm , Arm64 , Arm64ec , }
    };
}

TargetArch!();