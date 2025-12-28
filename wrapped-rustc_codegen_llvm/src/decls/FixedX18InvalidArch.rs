macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! FixedX18InvalidArch {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_fixed_x18_invalid_arch)] pub (crate) struct FixedX18InvalidArch < 'a > { pub arch : & 'a str , }
    };
}

FixedX18InvalidArch!();