macro_rules! deps {
    () => {
        ArgPredicate!();
        OsStr!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < S : Into < OsStr > > From < S > for ArgPredicate { fn from (other : S) -> Self { Self :: Equals (other . into ()) } }
    };
}

impl_67!();