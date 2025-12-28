macro_rules! deps {
    () => {
        RangedI64ValueParser!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < T : TryFrom < i64 > + Clone + Send + Sync , B : RangeBounds < i64 > > From < B > for RangedI64ValueParser < T > { fn from (range : B) -> Self { Self { bounds : (range . start_bound () . cloned () , range . end_bound () . cloned ()) , target : Default :: default () , } } }
    };
}

impl_292!()