macro_rules! VariantMask {
    () => {
        enum VariantMask { NoPadding = 2 , UrlSafe = 4 , }
    };
}

VariantMask!();