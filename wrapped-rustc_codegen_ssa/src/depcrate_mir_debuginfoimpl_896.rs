// Generated macro for impl_896 (impl)
macro_rules! Depcrate_mir_debuginfoimpl_896 {
() => {
// Module: crate::mir::debuginfo
// Provides: {"impl_896"}
// Dependencies: {}
impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > DebugInfoOffsetLocation < 'tcx , Bx > for PlaceRef < 'tcx , Bx :: Value > { fn deref (& self , bx : & mut Bx) -> Self { bx . load_operand (* self) . deref (bx . cx ()) } fn layout (& self) -> TyAndLayout < 'tcx > { self . layout } fn project_field (& self , bx : & mut Bx , field : FieldIdx) -> Self { PlaceRef :: project_field (* self , bx , field . index ()) } fn project_constant_index (& self , bx : & mut Bx , offset : u64) -> Self { let lloffset = bx . cx () . const_usize (offset) ; self . project_index (bx , lloffset) } fn downcast (& self , bx : & mut Bx , variant : VariantIdx) -> Self { self . project_downcast (bx , variant) } }
};
}
