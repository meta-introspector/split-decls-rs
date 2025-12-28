macro_rules! deps {
    () => {
        BuilderMethods!();
        DebugInfoOffsetLocation!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > DebugInfoOffsetLocation < 'tcx , Bx > for TyAndLayout < 'tcx > { fn deref (& self , bx : & mut Bx) -> Self { bx . cx () . layout_of (self . ty . builtin_deref (true) . unwrap_or_else (| | bug ! ("cannot deref `{}`" , self . ty)) ,) } fn layout (& self) -> TyAndLayout < 'tcx > { * self } fn project_field (& self , bx : & mut Bx , field : FieldIdx) -> Self { self . field (bx . cx () , field . index ()) } fn project_constant_index (& self , bx : & mut Bx , index : u64) -> Self { self . field (bx . cx () , index as usize) } fn downcast (& self , bx : & mut Bx , variant : VariantIdx) -> Self { self . for_variant (bx . cx () , variant) } }
    };
}

impl_473!()