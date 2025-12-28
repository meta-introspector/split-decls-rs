macro_rules! ExtraHeaders {
    () => {
        # [doc = " An iterator over extra headers in [owned][crate::Commit] and [borrowed][crate::CommitRef] commits."] pub struct ExtraHeaders < I > { inner : I , }
    };
}

ExtraHeaders!()