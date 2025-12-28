macro_rules! deps {
    () => {
        FluentArgs!();
        FluentValue!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'args , K , V > FromIterator < (K , V) > for FluentArgs < 'args > where K : Into < Cow < 'args , str > > , V : Into < FluentValue < 'args > > , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let iter = iter . into_iter () ; let mut args = if let Some (size) = iter . size_hint () . 1 { FluentArgs :: with_capacity (size) } else { FluentArgs :: new () } ; for (k , v) in iter { args . set (k , v) ; } args } }
    };
}

impl_2!();