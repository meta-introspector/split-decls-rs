macro_rules! deps {
    () => {
        Visitor!();
        FnRetTy!();
    };
}

macro_rules! walk_fn_ret_ty {
    () => {
        deps!();
        pub fn walk_fn_ret_ty < 'v , V : Visitor < 'v > > (visitor : & mut V , ret_ty : & 'v FnRetTy < 'v >) -> V :: Result { if let FnRetTy :: Return (output_ty) = * ret_ty { try_visit ! (visitor . visit_ty_unambig (output_ty)) ; } V :: Result :: output () }
    };
}

walk_fn_ret_ty!()