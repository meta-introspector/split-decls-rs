macro_rules! deps {
    () => {
        ArenaMap!();
        Idx!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T , V > FromIterator < (Idx < V > , T) > for ArenaMap < Idx < V > , T > { fn from_iter < I : IntoIterator < Item = (Idx < V > , T) > > (iter : I) -> Self { let mut this = Self :: new () ; this . extend (iter) ; this } }
    };
}

impl_6!();