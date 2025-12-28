macro_rules! deps {
    () => {
        Rewrites!();
        Platform!();
        Options!();
    };
}

macro_rules! RewriteOptions {
    () => {
        deps!();
        # [doc = " Options to configure how rewrites are tracked as part of the [`index()`](crate::index()) call."] pub struct RewriteOptions < 'a , Find > where Find : gix_object :: FindObjectOrHeader , { # [doc = " The cache to be used when rename-tracking by similarity is enabled, typically the default."] # [doc = " Note that it's recommended to call [`clear_resource_cache()`](`crate::blob::Platform::clear_resource_cache()`)"] # [doc = " between the calls to avoid runaway memory usage, as the cache isn't limited."] pub resource_cache : & 'a mut crate :: blob :: Platform , # [doc = " A way to lookup objects from the object database, for use in similarity checks."] pub find : & 'a Find , # [doc = " Configure how rewrites are tracked."] pub rewrites : crate :: Rewrites , }
    };
}

RewriteOptions!();