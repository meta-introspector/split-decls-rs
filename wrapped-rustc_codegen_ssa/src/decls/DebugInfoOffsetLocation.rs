macro_rules! DebugInfoOffsetLocation {
    () => {
        trait DebugInfoOffsetLocation < 'tcx , Bx > { fn deref (& self , bx : & mut Bx) -> Self ; fn layout (& self) -> TyAndLayout < 'tcx > ; fn project_field (& self , bx : & mut Bx , field : FieldIdx) -> Self ; fn project_constant_index (& self , bx : & mut Bx , offset : u64) -> Self ; fn downcast (& self , bx : & mut Bx , variant : VariantIdx) -> Self ; }
    };
}

DebugInfoOffsetLocation!();