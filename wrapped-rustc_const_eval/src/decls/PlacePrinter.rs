macro_rules! deps {
    () => {
        Place!();
        Machine!();
        InterpCx!();
    };
}

macro_rules! PlacePrinter {
    () => {
        deps!();
        # [doc (hidden)] # [doc = " Helper struct for the `dump_place` function."] pub struct PlacePrinter < 'a , 'tcx , M : Machine < 'tcx > > { ecx : & 'a InterpCx < 'tcx , M > , place : Place < M :: Provenance > , }
    };
}

PlacePrinter!()