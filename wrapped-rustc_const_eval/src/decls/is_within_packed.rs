macro_rules! deps {
    () => {
        Place!();
    };
}

macro_rules! is_within_packed {
    () => {
        deps!();
        pub fn is_within_packed < 'tcx , L > (tcx : TyCtxt < 'tcx > , local_decls : & L , place : Place < 'tcx > ,) -> Option < Align > where L : HasLocalDecls < 'tcx > , { place . iter_projections () . rev () . take_while (| (_base , elem) | ! matches ! (elem , ProjectionElem :: Deref)) . filter_map (| (base , _elem) | { base . ty (local_decls , tcx) . ty . ty_adt_def () . and_then (| adt | adt . repr () . pack) }) . min () }
    };
}

is_within_packed!();