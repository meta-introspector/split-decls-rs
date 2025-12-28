macro_rules! BitcodeSection {
    () => {
        # [doc = " What kind of llvm bitcode section to embed in an object file."] # [derive (Clone , Copy , PartialEq)] pub enum BitcodeSection { None , Full , }
    };
}

BitcodeSection!();