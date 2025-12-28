macro_rules! deps {
    () => {
        CheckParameters!();
    };
}

macro_rules! check_no_parameters_use {
    () => {
        deps!();
        # [doc = " Checks that function parameters aren't used in the function body."] fn check_no_parameters_use < 'tcx > (tcx : TyCtxt < 'tcx > , body : & 'tcx hir :: Body < 'tcx >) { let mut params = HirIdSet :: default () ; for param in body . params { param . pat . each_binding (| _binding_mode , hir_id , _span , _ident | { params . insert (hir_id) ; }) ; } CheckParameters { tcx , params } . visit_body (body) ; }
    };
}

check_no_parameters_use!()