macro_rules! find_param_in_ty {
    () => {
        # [doc = " Traverses the given ty (either a `ty::Ty` or a `ty::GenericArg`) and searches for references"] # [doc = " to the given `param_to_point_at`. Returns `true` if it finds any use of the param."] fn find_param_in_ty < 'tcx > (ty : ty :: GenericArg < 'tcx > , param_to_point_at : ty :: GenericArg < 'tcx > ,) -> bool { let mut walk = ty . walk () ; while let Some (arg) = walk . next () { if arg == param_to_point_at { return true ; } if let ty :: GenericArgKind :: Type (ty) = arg . kind () && let ty :: Alias (ty :: Projection | ty :: Inherent , ..) = ty . kind () { walk . skip_current_subtree () ; } } false }
    };
}

find_param_in_ty!();