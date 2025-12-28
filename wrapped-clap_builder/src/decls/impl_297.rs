macro_rules! deps {
    () => {
        RangedU64ValueParser!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < T : TryFrom < u64 > , B : RangeBounds < u64 > > From < B > for RangedU64ValueParser < T > { fn from (range : B) -> Self { Self { bounds : (range . start_bound () . cloned () , range . end_bound () . cloned ()) , target : Default :: default () , } } }
    };
}

impl_297!();