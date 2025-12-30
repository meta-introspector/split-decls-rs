// Generated macro for impl_962 (impl)
macro_rules! Depcrateimpl_962 {
() => {
// Module: crate
// Provides: {"impl_962"}
// Dependencies: {}
impl TypeVisitor < Interner > for PlaceholderCollector < '_ > { type BreakTy = () ; fn as_dyn (& mut self) -> & mut dyn TypeVisitor < Interner , BreakTy = Self :: BreakTy > { self } fn interner (& self) -> Interner { Interner } fn visit_ty (& mut self , ty : & Ty , outer_binder : DebruijnIndex ,) -> std :: ops :: ControlFlow < Self :: BreakTy > { let has_placeholder_bits = TypeFlags :: HAS_TY_PLACEHOLDER | TypeFlags :: HAS_CT_PLACEHOLDER ; let TyData { kind , flags } = ty . data (Interner) ; if let TyKind :: Placeholder (idx) = kind { self . collect (* idx) ; } else if flags . intersects (has_placeholder_bits) { return ty . super_visit_with (self , outer_binder) ; } else { } std :: ops :: ControlFlow :: Continue (()) } fn visit_const (& mut self , constant : & chalk_ir :: Const < Interner > , _outer_binder : DebruijnIndex ,) -> std :: ops :: ControlFlow < Self :: BreakTy > { if let chalk_ir :: ConstValue :: Placeholder (idx) = constant . data (Interner) . value { self . collect (idx) ; } std :: ops :: ControlFlow :: Continue (()) } }
};
}
