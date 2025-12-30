// Generated macro for get_atomic_name (function)
macro_rules! Depcrate_mutex_atomicget_atomic_name {
() => {
// Module: crate::mutex_atomic
// Provides: {"get_atomic_name"}
// Dependencies: {}
fn get_atomic_name (ty : Ty < '_ >) -> Option < & 'static str > { match ty . kind () { ty :: Bool => Some ("AtomicBool") , ty :: Uint (uint_ty) => { match uint_ty { UintTy :: U8 => Some ("AtomicU8") , UintTy :: U16 => Some ("AtomicU16") , UintTy :: U32 => Some ("AtomicU32") , UintTy :: U64 => Some ("AtomicU64") , UintTy :: Usize => Some ("AtomicUsize") , UintTy :: U128 => None , } } , ty :: Int (int_ty) => { match int_ty { IntTy :: I8 => Some ("AtomicI8") , IntTy :: I16 => Some ("AtomicI16") , IntTy :: I32 => Some ("AtomicI32") , IntTy :: I64 => Some ("AtomicI64") , IntTy :: Isize => Some ("AtomicIsize") , IntTy :: I128 => None , } } , ty :: RawPtr (_ , Mutability :: Mut) => Some ("AtomicPtr") , _ => None , } }
};
}
