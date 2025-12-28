macro_rules! deps {
    () => {
        FutureGroup!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < F : Future > Extend < F > for FutureGroup < F > { fn extend < T : IntoIterator < Item = F > > (& mut self , iter : T) { let iter = iter . into_iter () ; let len = iter . size_hint () . 1 . unwrap_or_default () ; self . reserve (len) ; for future in iter { self . insert (future) ; } } }
    };
}

impl_205!()