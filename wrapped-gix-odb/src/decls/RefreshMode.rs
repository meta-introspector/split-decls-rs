macro_rules! RefreshMode {
    () => {
        # [doc = " Decide what happens when all indices are loaded."] # [derive (Default , Clone , Copy)] pub enum RefreshMode { # [doc = " Check for new or changed pack indices (and pack data files) when the last known index is loaded."] # [doc = " During runtime we will keep pack indices stable by never reusing them, however, there is the option for"] # [doc = " clearing internal caches which is likely to change pack ids and it will trigger unloading of packs as they are missing on disk."] # [default] AfterAllIndicesLoaded , # [doc = " Use this if you expect a lot of missing objects that shouldn't trigger refreshes even after all packs are loaded."] # [doc = " This comes at the risk of not learning that the packs have changed in the mean time."] Never , }
    };
}

RefreshMode!();