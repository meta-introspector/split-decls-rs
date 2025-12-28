macro_rules! construct_place_string {
    () => {
        fn construct_place_string < 'tcx > (tcx : TyCtxt < '_ > , place : & Place < 'tcx >) -> String { let variable_name = match place . base { PlaceBase :: Upvar (upvar_id) => var_name (tcx , upvar_id . var_path . hir_id) . to_string () , _ => bug ! ("Capture_information should only contain upvars") , } ; let mut projections_str = String :: new () ; for (i , item) in place . projections . iter () . enumerate () { let proj = match item . kind { ProjectionKind :: Field (a , b) => format ! ("({a:?}, {b:?})") , ProjectionKind :: Deref => String :: from ("Deref") , ProjectionKind :: Index => String :: from ("Index") , ProjectionKind :: Subslice => String :: from ("Subslice") , ProjectionKind :: OpaqueCast => String :: from ("OpaqueCast") , ProjectionKind :: UnwrapUnsafeBinder => String :: from ("UnwrapUnsafeBinder") , } ; if i != 0 { projections_str . push (',') ; } projections_str . push_str (proj . as_str ()) ; } format ! ("{variable_name}[{projections_str}]") }
    };
}

construct_place_string!();