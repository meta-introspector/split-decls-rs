macro_rules! deps {
    () => {
        Place!();
        ProjectionStore!();
        PlaceElem!();
    };
}

macro_rules! impl_878 {
    () => {
        deps!();
        impl < 'db > Place < 'db > { fn is_parent (& self , child : & Place < 'db > , store : & ProjectionStore < 'db >) -> bool { self . local == child . local && child . projection . lookup (store) . starts_with (self . projection . lookup (store)) } # [doc = " The place itself is not included"] fn iterate_over_parents < 'a > (& 'a self , store : & 'a ProjectionStore < 'db > ,) -> impl Iterator < Item = Place < 'db > > + 'a { let projection = self . projection . lookup (store) ; (0 .. projection . len ()) . map (| x | & projection [0 .. x]) . filter_map (move | x | { Some (Place { local : self . local , projection : store . intern_if_exist (x) ? }) }) } fn project (& self , projection : PlaceElem < 'db > , store : & mut ProjectionStore < 'db >) -> Place < 'db > { Place { local : self . local , projection : self . projection . project (projection , store) } } }
    };
}

impl_878!();