macro_rules! RandomState {
    () => {
        # [doc = " A [`BuildHasher`] for [`quality::FoldHasher`](FoldHasher) that is randomly initialized."] # [derive (Clone , Default , Debug)] pub struct RandomState { inner : fast :: RandomState , }
    };
}

RandomState!();