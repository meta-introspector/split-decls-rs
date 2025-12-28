macro_rules! ObjectCache {
    () => {
        # [doc = " A type to store object caches in boxes."] pub type ObjectCache = dyn gix_pack :: cache :: Object + Send + 'static ;
    };
}

ObjectCache!()