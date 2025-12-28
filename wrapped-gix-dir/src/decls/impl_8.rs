macro_rules! deps {
    () => {
        PathspecMatch!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < gix_pathspec :: search :: Match < '_ > > for PathspecMatch { fn from (m : gix_pathspec :: search :: Match < '_ >) -> Self { if m . is_excluded () { PathspecMatch :: Excluded } else { m . kind . into () } } }
    };
}

impl_8!()