macro_rules! PathValue {
    () => {
        # [derive (Clone)] pub struct PathValue (pub PathBuf) ;
    };
}

PathValue!()