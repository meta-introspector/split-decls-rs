macro_rules! deps {
    () => {
        IsPrefixOf!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl < 'tcx > IsPrefixOf < 'tcx > for PlaceRef < 'tcx > { fn is_prefix_of (& self , other : PlaceRef < 'tcx >) -> bool { self . local == other . local && self . projection . len () <= other . projection . len () && self . projection == & other . projection [.. self . projection . len ()] } }
    };
}

impl_307!()