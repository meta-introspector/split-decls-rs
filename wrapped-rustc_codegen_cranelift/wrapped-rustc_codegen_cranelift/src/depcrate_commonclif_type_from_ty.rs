// Generated macro for clif_type_from_ty (function)
macro_rules! Depcrate_commonclif_type_from_ty {
() => {
// Module: crate::common
// Provides: {"clif_type_from_ty"}
// Dependencies: {}
fn clif_type_from_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Option < types :: Type > { Some (match ty . kind () { ty :: Bool => types :: I8 , ty :: Uint (size) => match size { UintTy :: U8 => types :: I8 , UintTy :: U16 => types :: I16 , UintTy :: U32 => types :: I32 , UintTy :: U64 => types :: I64 , UintTy :: U128 => types :: I128 , UintTy :: Usize => pointer_ty (tcx) , } , ty :: Int (size) => match size { IntTy :: I8 => types :: I8 , IntTy :: I16 => types :: I16 , IntTy :: I32 => types :: I32 , IntTy :: I64 => types :: I64 , IntTy :: I128 => types :: I128 , IntTy :: Isize => pointer_ty (tcx) , } , ty :: Char => types :: I32 , ty :: Float (size) => match size { FloatTy :: F16 => types :: F16 , FloatTy :: F32 => types :: F32 , FloatTy :: F64 => types :: F64 , FloatTy :: F128 => types :: F128 , } , ty :: FnPtr (..) => pointer_ty (tcx) , ty :: RawPtr (pointee_ty , _) | ty :: Ref (_ , pointee_ty , _) => { if tcx . type_has_metadata (* pointee_ty , ty :: TypingEnv :: fully_monomorphized ()) { return None ; } else { pointer_ty (tcx) } } ty :: Param (_) => bug ! ("ty param {:?}" , ty) , _ => return None , }) }
};
}
