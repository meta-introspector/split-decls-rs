macro_rules! CachedLlbb {
    () => {
        enum CachedLlbb < T > { # [doc = " Nothing created yet."] None , # [doc = " Has been created."] Some (T) , # [doc = " Nothing created yet, and nothing should be."] Skip , }
    };
}

CachedLlbb!()