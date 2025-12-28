macro_rules! macho_is_arm64e {
    () => {
        # [doc = " Is Apple's CPU subtype `arm64e`s"] fn macho_is_arm64e (target : & Target) -> bool { target . llvm_target . starts_with ("arm64e") }
    };
}

macho_is_arm64e!()