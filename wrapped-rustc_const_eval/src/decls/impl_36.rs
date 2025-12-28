macro_rules! deps {
    () => {
        InteriorMutableBorrowEscaping!();
        DiagImportance!();
        ConstCx!();
        NonConstOp!();
        EscapingCellBorrow!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for EscapingCellBorrow { fn importance (& self) -> DiagImportance { DiagImportance :: Secondary } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: InteriorMutableBorrowEscaping { span , kind : ccx . const_kind () }) } }
    };
}

impl_36!()