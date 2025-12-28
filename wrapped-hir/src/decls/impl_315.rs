macro_rules! deps {
    () => {
        Type!();
        BuiltinType!();
        Crate!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl BuiltinType { pub fn str () -> BuiltinType { BuiltinType { inner : hir_def :: builtin_type :: BuiltinType :: Str } } pub fn i32 () -> BuiltinType { BuiltinType { inner : hir_def :: builtin_type :: BuiltinType :: Int (hir_ty :: primitive :: BuiltinInt :: I32) , } } pub fn ty < 'db > (self , db : & 'db dyn HirDatabase) -> Type < 'db > { let core = Crate :: core (db) . map (| core | core . id) . unwrap_or_else (| | db . all_crates () [0]) ; let interner = DbInterner :: new_with (db , None , None) ; Type :: new_for_crate (core , Ty :: from_builtin_type (interner , self . inner)) } pub fn name (self) -> Name { self . inner . as_name () } pub fn is_int (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Int (_)) } pub fn is_uint (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Uint (_)) } pub fn is_float (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Float (_)) } pub fn is_f16 (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Float (hir_def :: builtin_type :: BuiltinFloat :: F16)) } pub fn is_f32 (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Float (hir_def :: builtin_type :: BuiltinFloat :: F32)) } pub fn is_f64 (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Float (hir_def :: builtin_type :: BuiltinFloat :: F64)) } pub fn is_f128 (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Float (hir_def :: builtin_type :: BuiltinFloat :: F128)) } pub fn is_char (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Char) } pub fn is_bool (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Bool) } pub fn is_str (& self) -> bool { matches ! (self . inner , hir_def :: builtin_type :: BuiltinType :: Str) } }
    };
}

impl_315!()