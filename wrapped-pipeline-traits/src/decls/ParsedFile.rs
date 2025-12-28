macro_rules! ParsedFile {
    () => {
        # [derive (Clone)] pub struct ParsedFile (pub String , pub PathBuf) ;
    };
}

ParsedFile!();