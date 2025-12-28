macro_rules! deps {
    () => {
        HasMutInterior!();
        NeedsDrop!();
        ConstCx!();
        NeedsNonConstDrop!();
    };
}

macro_rules! in_any_value_of_ty {
    () => {
        deps!();
        pub fn in_any_value_of_ty < 'tcx > (cx : & ConstCx < '_ , 'tcx > , ty : Ty < 'tcx > , tainted_by_errors : Option < ErrorGuaranteed > ,) -> ConstQualifs { ConstQualifs { has_mut_interior : HasMutInterior :: in_any_value_of_ty (cx , ty) , needs_drop : NeedsDrop :: in_any_value_of_ty (cx , ty) , needs_non_const_drop : NeedsNonConstDrop :: in_any_value_of_ty (cx , ty) , tainted_by_errors , } }
    };
}

in_any_value_of_ty!()