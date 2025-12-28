macro_rules! PackCache {
    () => {
        # [doc = " A type to store pack caches in boxes."] pub type PackCache = dyn gix_pack :: cache :: DecodeEntry + Send + 'static ;
    };
}

PackCache!()