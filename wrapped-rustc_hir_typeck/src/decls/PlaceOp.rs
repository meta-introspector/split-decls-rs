macro_rules! PlaceOp {
    () => {
        # [derive (Debug , Copy , Clone)] pub enum PlaceOp { Deref , Index , }
    };
}

PlaceOp!();