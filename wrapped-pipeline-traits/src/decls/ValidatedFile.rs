macro_rules! ValidatedFile {
    () => {
        # [derive (Debug , Clone)] pub struct ValidatedFile (pub String , pub PathBuf) ;
    };
}

ValidatedFile!();