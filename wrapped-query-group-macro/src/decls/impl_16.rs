macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Lookup { pub (crate) fn prepare_signature (& mut self) { let sig = & self . signature ; let ident = format_ident ! ("lookup_{}" , sig . ident) ; let ty = self . pat_and_tys . to_vec () ; let interned_key = & self . return_ty ; let interned_pat = ty . first () . expect ("at least one pat; this is a bug") ; let interned_return_ty = & interned_pat . ty ; self . signature = parse_quote ! (fn # ident (& self , id : # interned_key) -> # interned_return_ty) ; } }
    };
}

impl_16!()