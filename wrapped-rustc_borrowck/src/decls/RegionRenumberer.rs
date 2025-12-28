macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
    };
}

macro_rules! RegionRenumberer {
    () => {
        deps!();
        struct RegionRenumberer < 'a , 'tcx > { infcx : & 'a BorrowckInferCtxt < 'tcx > , }
    };
}

RegionRenumberer!()