macro_rules! deps {
    () => {
        Scalar!();
        BackendRepr!();
        LayoutData!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < FieldIdx : Idx , VariantIdx : Idx > LayoutData < FieldIdx , VariantIdx > { # [doc = " Returns `true` if the layout corresponds to an unsized type."] # [inline] pub fn is_unsized (& self) -> bool { self . backend_repr . is_unsized () } # [inline] pub fn is_sized (& self) -> bool { self . backend_repr . is_sized () } # [doc = " Returns `true` if the type is sized and a 1-ZST (meaning it has size 0 and alignment 1)."] pub fn is_1zst (& self) -> bool { self . is_sized () && self . size . bytes () == 0 && self . align . abi . bytes () == 1 } # [doc = " Returns `true` if the type is a ZST and not unsized."] # [doc = ""] # [doc = " Note that this does *not* imply that the type is irrelevant for layout! It can still have"] # [doc = " non-trivial alignment constraints. You probably want to use `is_1zst` instead."] pub fn is_zst (& self) -> bool { match self . backend_repr { BackendRepr :: Scalar (_) | BackendRepr :: ScalarPair (..) | BackendRepr :: SimdVector { .. } => false , BackendRepr :: Memory { sized } => sized && self . size . bytes () == 0 , } } # [doc = " Checks if these two `Layout` are equal enough to be considered \"the same for all function"] # [doc = " call ABIs\". Note however that real ABIs depend on more details that are not reflected in the"] # [doc = " `Layout`; the `PassMode` need to be compared as well. Also note that we assume"] # [doc = " aggregates are passed via `PassMode::Indirect` or `PassMode::Cast`; more strict"] # [doc = " checks would otherwise be required."] pub fn eq_abi (& self , other : & Self) -> bool { self . size == other . size && self . is_sized () == other . is_sized () && self . backend_repr . eq_up_to_validity (& other . backend_repr) && self . backend_repr . is_bool () == other . backend_repr . is_bool () && self . align . abi == other . align . abi && self . max_repr_align == other . max_repr_align && self . unadjusted_abi_align == other . unadjusted_abi_align } }
    };
}

impl_72!()