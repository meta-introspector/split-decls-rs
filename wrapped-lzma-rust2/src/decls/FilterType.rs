macro_rules! FilterType {
    () => {
        # [doc = " Supported filter types in XZ format."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum FilterType { # [doc = " Delta filter"] Delta , # [doc = " BCJ x86 filter"] BcjX86 , # [doc = " BCJ PowerPC filter"] BcjPpc , # [doc = " BCJ IA64 filter"] BcjIa64 , # [doc = " BCJ ARM filter"] BcjArm , # [doc = " BCJ ARM Thumb"] BcjArmThumb , # [doc = " BCJ SPARC filter"] BcjSparc , # [doc = " BCJ ARM64 filter"] BcjArm64 , # [doc = " BCJ RISC-V filter"] BcjRiscv , # [doc = " LZMA2 filter"] Lzma2 , }
    };
}

FilterType!()