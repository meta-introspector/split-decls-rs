macro_rules! construct_path_string {
    () => {
        fn construct_path_string < 'tcx > (tcx : TyCtxt < '_ > , place : & Place < 'tcx >) -> String { let place_str = construct_place_string (tcx , place) ; format ! ("{place_str} used here") }
    };
}

construct_path_string!()