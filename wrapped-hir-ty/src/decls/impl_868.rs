macro_rules! deps {
    () => {
        Operand!();
        OperandKind!();
        MemoryMap!();
        HirDatabase!();
    };
}

macro_rules! impl_868 {
    () => {
        deps!();
        impl < 'db > Operand < 'db > { fn from_concrete_const (data : Box < [u8] > , memory_map : MemoryMap < 'db > , ty : Ty < 'db >) -> Self { let interner = DbInterner :: conjure () ; Operand { kind : OperandKind :: Constant { konst : Const :: new_valtree (interner , ty , data , memory_map) , ty , } , span : None , } } fn from_bytes (data : Box < [u8] > , ty : Ty < 'db >) -> Self { Operand :: from_concrete_const (data , MemoryMap :: default () , ty) } fn const_zst (ty : Ty < 'db >) -> Operand < 'db > { Self :: from_bytes (Box :: default () , ty) } fn from_fn (db : & 'db dyn HirDatabase , func_id : hir_def :: FunctionId , generic_args : GenericArgs < 'db > ,) -> Operand < 'db > { let interner = DbInterner :: new_with (db , None , None) ; let ty = Ty :: new_fn_def (interner , CallableDefId :: FunctionId (func_id) . into () , generic_args) ; Operand :: from_bytes (Box :: default () , ty) } }
    };
}

impl_868!();