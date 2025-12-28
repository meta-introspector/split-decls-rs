macro_rules! deps {
    () => {
        Ty!();
        Sp!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl Ty { pub (crate) fn from_syn_ty (ty : & Type) -> Sp < Self > { use self :: Ty :: { Option , OptionOption , OptionVec , OptionVecVec , Other , Unit , Vec , VecVec } ; let t = | kind | Sp :: new (kind , ty . span ()) ; if is_unit_ty (ty) { t (Unit) } else if let Some (vt) = get_vec_ty (ty , Vec , VecVec) { t (vt) } else if let Some (subty) = subty_if_name (ty , "Option") { if is_generic_ty (subty , "Option") { t (OptionOption) } else if let Some (vt) = get_vec_ty (subty , OptionVec , OptionVecVec) { t (vt) } else { t (Option) } } else { t (Other) } } pub (crate) fn as_str (& self) -> & 'static str { match self { Self :: Unit => "()" , Self :: Vec => "Vec<T>" , Self :: Option => "Option<T>" , Self :: OptionOption => "Option<Option<T>>" , Self :: OptionVec => "Option<Vec<T>>" , Self :: VecVec => "Vec<Vec<T>>" , Self :: OptionVecVec => "Option<Vec<Vec<T>>>" , Self :: Other => "...other..." , } } }
    };
}

impl_103!();