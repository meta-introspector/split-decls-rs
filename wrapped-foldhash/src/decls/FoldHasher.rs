macro_rules! deps {
    () => {
        RandomState!();
        SeedableRandomState!();
        FixedState!();
    };
}

macro_rules! FoldHasher {
    () => {
        deps!();
        # [doc = " A [`Hasher`] instance implementing foldhash, optimized for quality."] # [doc = ""] # [doc = " While you can create one directly with [`FoldHasher::with_seed`], you"] # [doc = " most likely want to use [`RandomState`], [`SeedableRandomState`] or"] # [doc = " [`FixedState`] to create [`FoldHasher`]s."] # [derive (Clone)] pub struct FoldHasher < 'a > { pub (crate) inner : fast :: FoldHasher < 'a > , }
    };
}

FoldHasher!();