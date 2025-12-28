macro_rules! deps {
    () => {
        AllLocalUsesVisitor!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for AllLocalUsesVisitor { fn visit_local (& mut self , local : Local , _context : PlaceContext , location : Location) { if local == self . for_local { self . uses . insert (location) ; } } }
    };
}

impl_67!();