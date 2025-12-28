macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! inner_type {
    () => {
        deps!();
        pub (crate) fn inner_type (field_ty : & Type) -> & Type { let ty = Ty :: from_syn_ty (field_ty) ; match * ty { Ty :: Vec | Ty :: Option => sub_type (field_ty) . unwrap_or (field_ty) , Ty :: OptionOption | Ty :: OptionVec | Ty :: VecVec => { sub_type (field_ty) . and_then (sub_type) . unwrap_or (field_ty) } Ty :: OptionVecVec => sub_type (field_ty) . and_then (sub_type) . and_then (sub_type) . unwrap_or (field_ty) , _ => field_ty , } }
    };
}

inner_type!()