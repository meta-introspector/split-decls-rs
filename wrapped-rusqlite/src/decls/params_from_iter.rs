macro_rules! deps {
    () => {
        ParamsFromIter!();
        ToSql!();
    };
}

macro_rules! params_from_iter {
    () => {
        deps!();
        # [doc = " Constructor function for a [`ParamsFromIter`]. See its documentation for"] # [doc = " more."] # [inline] pub fn params_from_iter < I > (iter : I) -> ParamsFromIter < I > where I : IntoIterator , I :: Item : ToSql , { ParamsFromIter (iter) }
    };
}

params_from_iter!()