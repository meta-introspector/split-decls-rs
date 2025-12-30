// Generated macro for impl_755 (impl)
macro_rules! Depcrate_mirimpl_755 {
() => {
// Module: crate::mir
// Provides: {"impl_755"}
// Dependencies: {}
impl Operand { fn from_concrete_const (data : Box < [u8] > , memory_map : MemoryMap , ty : Ty) -> Self { Operand { kind : OperandKind :: Constant (intern_const_scalar (ConstScalar :: Bytes (data , memory_map) , ty ,)) , span : None , } } fn from_bytes (data : Box < [u8] > , ty : Ty) -> Self { Operand :: from_concrete_const (data , MemoryMap :: default () , ty) } fn const_zst (ty : Ty) -> Operand { Self :: from_bytes (Box :: default () , ty) } fn from_fn (db : & dyn HirDatabase , func_id : hir_def :: FunctionId , generic_args : Substitution ,) -> Operand { let ty = chalk_ir :: TyKind :: FnDef (CallableDefId :: FunctionId (func_id) . to_chalk (db) , generic_args) . intern (Interner) ; Operand :: from_bytes (Box :: default () , ty) } }
};
}
