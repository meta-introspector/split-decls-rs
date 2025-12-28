macro_rules! deps {
    () => {
        PathspecMatch!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl PathspecMatch { pub (crate) fn should_ignore (& self) -> bool { match self { PathspecMatch :: Always | PathspecMatch :: Excluded => true , PathspecMatch :: Prefix | PathspecMatch :: WildcardMatch | PathspecMatch :: Verbatim => false , } } }
    };
}

impl_6!();