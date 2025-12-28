macro_rules! TargetDirectoryConfig {
    () => {
        # [derive (Clone , Debug , Default , PartialEq , Eq)] pub enum TargetDirectoryConfig { # [default] None , UseSubdirectory , Directory (Utf8PathBuf) , }
    };
}

TargetDirectoryConfig!();