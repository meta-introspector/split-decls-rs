macro_rules! deps {
    () => {
        Url!();
        Scheme!();
        Error!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        # [doc = " Builder"] impl Url { # [doc = " Enable alternate serialization for this url, e.g. `file:///path` becomes `/path`."] # [doc = ""] # [doc = " This is automatically set correctly for parsed URLs, but can be set here for urls"] # [doc = " created by constructor."] pub fn serialize_alternate_form (mut self , use_alternate_form : bool) -> Self { self . serialize_alternative_form = use_alternate_form ; self } # [doc = " Turn a file url like `file://relative` into `file:///root/relative`, hence it assures the url's path component is absolute,"] # [doc = " using `current_dir` if needed to achieve that."] pub fn canonicalize (& mut self , current_dir : & std :: path :: Path) -> Result < () , gix_path :: realpath :: Error > { if self . scheme == Scheme :: File { let path = gix_path :: from_bstr (Cow :: Borrowed (self . path . as_ref ())) ; let abs_path = gix_path :: realpath_opts (path . as_ref () , current_dir , gix_path :: realpath :: MAX_SYMLINKS) ? ; self . path = gix_path :: into_bstr (abs_path) . into_owned () ; } Ok (()) } }
    };
}

impl_50!();