macro_rules! Base64Variant {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq)] enum Base64Variant { Original = 1 , OriginalNoPadding = 3 , UrlSafe = 5 , UrlSafeNoPadding = 7 , }
    };
}

Base64Variant!()