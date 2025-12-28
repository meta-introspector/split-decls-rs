macro_rules! deps {
    () => {
        ConstCx!();
        DiagImportance!();
        MutableBorrowEscaping!();
        EscapingMutBorrow!();
        NonConstOp!();
        Status!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for EscapingMutBorrow { fn status_in_item (& self , _ccx : & ConstCx < '_ , 'tcx >) -> Status { Status :: Forbidden } fn importance (& self) -> DiagImportance { DiagImportance :: Secondary } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: MutableBorrowEscaping { span , kind : ccx . const_kind () }) } }
    };
}

impl_38!()