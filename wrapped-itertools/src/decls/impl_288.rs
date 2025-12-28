macro_rules! deps {
    () => {
        GroupingMapFn!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < F > std :: fmt :: Debug for GroupingMapFn < F > { debug_fmt_fields ! (GroupingMapFn ,) ; }
    };
}

impl_288!();