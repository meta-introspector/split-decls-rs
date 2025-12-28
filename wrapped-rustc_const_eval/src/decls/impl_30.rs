macro_rules! deps {
    () => {
        HeapAllocation!();
        ConstCx!();
        UnallowedHeapAllocations!();
        NonConstOp!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for HeapAllocation { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: UnallowedHeapAllocations { span , kind : ccx . const_kind () , teach : ccx . tcx . sess . teach (E0010) , }) } }
    };
}

impl_30!()