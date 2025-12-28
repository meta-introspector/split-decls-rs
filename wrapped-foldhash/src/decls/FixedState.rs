macro_rules! FixedState {
    () => {
        # [doc = " A [`BuildHasher`] for [`quality::FoldHasher`](FoldHasher) that always has the same fixed seed."] # [doc = ""] # [doc = " Not recommended unless you absolutely need determinism."] # [derive (Clone , Default , Debug)] pub struct FixedState { inner : fast :: FixedState , }
    };
}

FixedState!()