macro_rules! deps {
    () => {
        IndexFileBundle!();
        MultiIndexFileBundle!();
    };
}

macro_rules! IndexAndPacks {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) enum IndexAndPacks { Index (IndexFileBundle) , # [doc = " Note that there can only be one multi-pack file per repository, but thanks to git alternates, there can be multiple overall."] MultiIndex (MultiIndexFileBundle) , }
    };
}

IndexAndPacks!();