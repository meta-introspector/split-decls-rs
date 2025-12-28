macro_rules! deps {
    () => {
        CacheKey!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl Default for CacheKey { fn default () -> Self { CacheKey { id : gix_hash :: Kind :: Sha1 . null () , use_id : false , is_link : false , location : BString :: default () , } } }
    };
}

impl_114!();