macro_rules! deps {
    () => {
        Source!();
        Metadata!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        # [doc = " Instantiation"] impl Metadata { # [doc = " Return metadata indicating the source of a [`File`][crate::File] is from an API user."] pub fn api () -> Self { file :: Metadata { path : None , source : Source :: Api , level : 0 , trust : gix_sec :: Trust :: Full , } } # [doc = " Return metadata as derived from the given `path` at `source`, which will also be used to derive the trust level"] # [doc = " by checking its ownership."] pub fn try_from_path (path : impl Into < PathBuf > , source : Source) -> std :: io :: Result < Self > { let path = path . into () ; gix_sec :: Trust :: from_path_ownership (& path) . map (| trust | Metadata { path : path . into () , source , level : 0 , trust , }) } # [doc = " Set the trust level of this instance to the given `trust` and return it."] # [doc = ""] # [doc = " Useful in conjunction with `Metadata::from(source)`."] pub fn with (mut self , trust : gix_sec :: Trust) -> Self { self . trust = trust ; self } # [doc = " Set the metadata to be located at the given `path`."] pub fn at (mut self , path : impl Into < PathBuf >) -> Self { self . path = Some (path . into ()) ; self } }
    };
}

impl_69!();