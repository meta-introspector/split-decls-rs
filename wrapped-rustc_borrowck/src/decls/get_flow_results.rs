macro_rules! deps {
    () => {
        BorrowckDomain!();
        BorrowSet!();
        RegionInferenceContext!();
        Borrows!();
        Borrowck!();
    };
}

macro_rules! get_flow_results {
    () => {
        deps!();
        fn get_flow_results < 'a , 'tcx > (tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx > , borrow_set : & 'a BorrowSet < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > ,) -> (Borrowck < 'a , 'tcx > , Results < BorrowckDomain >) { let borrows = Borrows :: new (tcx , body , regioncx , borrow_set) . iterate_to_fixpoint (tcx , body , Some ("borrowck") ,) ; let uninits = MaybeUninitializedPlaces :: new (tcx , body , move_data) . iterate_to_fixpoint (tcx , body , Some ("borrowck") ,) ; let ever_inits = EverInitializedPlaces :: new (body , move_data) . iterate_to_fixpoint (tcx , body , Some ("borrowck") ,) ; let analysis = Borrowck { borrows : borrows . analysis , uninits : uninits . analysis , ever_inits : ever_inits . analysis , } ; assert_eq ! (borrows . results . len () , uninits . results . len ()) ; assert_eq ! (borrows . results . len () , ever_inits . results . len ()) ; let results : Results < _ > = itertools :: izip ! (borrows . results , uninits . results , ever_inits . results) . map (| (borrows , uninits , ever_inits) | BorrowckDomain { borrows , uninits , ever_inits }) . collect () ; (analysis , results) }
    };
}

get_flow_results!()