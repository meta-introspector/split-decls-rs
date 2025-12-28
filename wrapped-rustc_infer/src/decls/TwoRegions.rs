macro_rules! TwoRegions {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Hash)] pub (crate) struct TwoRegions < 'tcx > { a : Region < 'tcx > , b : Region < 'tcx > , }
    };
}

TwoRegions!()