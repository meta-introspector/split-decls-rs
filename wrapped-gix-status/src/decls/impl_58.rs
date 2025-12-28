macro_rules! deps {
    () => {
        SymlinkCheck!();
        Error!();
        Delegate!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl SymlinkCheck { # [doc = " Create a new stack that starts operating at `root`."] pub fn new (root : PathBuf) -> Self { Self { inner : gix_fs :: Stack :: new (root) , } } # [doc = " Return a valid filesystem path located in our root by appending `relative_path`, which is guaranteed to"] # [doc = " not pass through a symbolic link. That way the caller can be sure to not be misled by an attacker that"] # [doc = " tries to make us reach outside of the repository."] # [doc = ""] # [doc = " Note that the file pointed to by `relative_path` may still be a symbolic link, or not exist at all,"] # [doc = " and that an error may also be produced if directories on the path leading to the leaf"] # [doc = " component of `relative_path` are missing."] # [doc = ""] # [doc = " ### Note"] # [doc = ""] # [doc = " On windows, no verification is performed, instead only the combined path is provided as usual."] pub fn verified_path (& mut self , relative_path : impl ToNormalPathComponents) -> std :: io :: Result < & Path > { self . inner . make_relative_path_current (relative_path , & mut Delegate) ? ; Ok (self . inner . current ()) } # [doc = " Like [`Self::verified_path()`], but do not fail if there is no directory entry at `relative_path` or on the way"] # [doc = " to `relative_path`. Instead."] # [doc = " For convenience, this incarnation is tuned to be easy to use with Git paths, i.e. slash-separated `BString` path."] pub fn verified_path_allow_nonexisting (& mut self , relative_path : & BStr) -> std :: io :: Result < Cow < '_ , Path > > { let rela_path = gix_path :: try_from_bstr (relative_path) . map_err (std :: io :: Error :: other) ? ; if let Err (err) = self . verified_path (rela_path . as_ref ()) { if err . kind () == std :: io :: ErrorKind :: NotFound { Ok (Cow :: Owned (self . inner . root () . join (rela_path))) } else { Err (err) } } else { Ok (Cow :: Borrowed (self . inner . current ())) } } }
    };
}

impl_58!()