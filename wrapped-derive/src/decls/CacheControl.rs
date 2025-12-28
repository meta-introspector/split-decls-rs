macro_rules! CacheControl {
    () => {
        # [derive (FromMeta , Clone)] # [darling (default)] pub struct CacheControl { public : bool , private : bool , pub no_cache : bool , pub max_age : usize , }
    };
}

CacheControl!();