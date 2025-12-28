macro_rules! SourceRootConfig {
    () => {
        # [derive (Default , Debug)] pub struct SourceRootConfig { pub fsc : FileSetConfig , pub local_filesets : Vec < u64 > , }
    };
}

SourceRootConfig!()