macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! get_vec_ty {
    () => {
        deps!();
        # [cfg (not (feature = "unstable-v5"))] fn get_vec_ty (ty : & Type , vec_ty : Ty , _vecvec_ty : Ty) -> Option < Ty > { is_generic_ty (ty , "Vec") . then_some (vec_ty) }
    };
}

get_vec_ty!()