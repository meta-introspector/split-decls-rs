macro_rules! deps {
    () => {
        NonConstOp!();
        InlineAsm!();
        UnallowedInlineAsm!();
        ConstCx!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for InlineAsm { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: UnallowedInlineAsm { span , kind : ccx . const_kind () }) } }
    };
}

impl_32!();