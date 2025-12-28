macro_rules! deps {
    () => {
        TypeKind!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl TypeKind { pub (crate) fn to_generic (self) -> rustc_codegen_ssa :: common :: TypeKind { use rustc_codegen_ssa :: common :: TypeKind as Common ; match self { Self :: Void => Common :: Void , Self :: Half => Common :: Half , Self :: Float => Common :: Float , Self :: Double => Common :: Double , Self :: X86_FP80 => Common :: X86_FP80 , Self :: FP128 => Common :: FP128 , Self :: PPC_FP128 => Common :: PPC_FP128 , Self :: Label => Common :: Label , Self :: Integer => Common :: Integer , Self :: Function => Common :: Function , Self :: Struct => Common :: Struct , Self :: Array => Common :: Array , Self :: Pointer => Common :: Pointer , Self :: Vector => Common :: Vector , Self :: Metadata => Common :: Metadata , Self :: Token => Common :: Token , Self :: ScalableVector => Common :: ScalableVector , Self :: BFloat => Common :: BFloat , Self :: X86_AMX => Common :: X86_AMX , } } }
    };
}

impl_445!();