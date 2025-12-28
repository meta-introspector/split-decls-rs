macro_rules! AbsPathBuf {
    () => {
        # [doc = " A [`Utf8PathBuf`] that is guaranteed to be absolute."] # [derive (Debug , Clone , Ord , PartialOrd , Eq , Hash)] pub struct AbsPathBuf (Utf8PathBuf) ;
    };
}

AbsPathBuf!();