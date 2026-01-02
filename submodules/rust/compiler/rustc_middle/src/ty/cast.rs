mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use crate :: mir ;}
mkuse!{use crate :: ty :: { self , Ty } ;}
mkitem!{mkenum!{# [doc = " Types that are represented as ints."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum IntTy { U (ty :: UintTy) , I , CEnum , Bool , Char , }}}
mkitem!{mkimpl!{impl IntTy { pub fn is_signed (self) -> bool { matches ! (self , Self :: I) } }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum CastTy < 'tcx > { # [doc = " Various types that are represented as ints and handled mostly"] # [doc = " in the same way, merged for easier matching."] Int (IntTy) , # [doc = " Floating-point types."] Float , # [doc = " Function pointers."] FnPtr , # [doc = " Raw pointers."] Ptr (ty :: TypeAndMut < 'tcx >) , }}}
mkitem!{mkenum!{# [doc = " Cast Kind. See [RFC 401](https://rust-lang.github.io/rfcs/0401-coercions.html)"] # [doc = " (or rustc_hir_analysis/check/cast.rs)."] # [derive (Copy , Clone , Debug , TyEncodable , TyDecodable , HashStable)] pub enum CastKind { PtrPtrCast , PtrAddrCast , AddrPtrCast , NumericCast , EnumCast , PrimIntCast , U8CharCast , ArrayPtrCast , FnPtrPtrCast , FnPtrAddrCast , }}}
mkitem!{mkimpl!{impl < 'tcx > CastTy < 'tcx > { # [doc = " Returns `Some` for integral/pointer casts."] # [doc = " Casts like unsizing casts will return `None`."] pub fn from_ty (t : Ty < 'tcx >) -> Option < CastTy < 'tcx > > { match * t . kind () { ty :: Bool => Some (CastTy :: Int (IntTy :: Bool)) , ty :: Char => Some (CastTy :: Int (IntTy :: Char)) , ty :: Int (_) => Some (CastTy :: Int (IntTy :: I)) , ty :: Infer (ty :: InferTy :: IntVar (_)) => Some (CastTy :: Int (IntTy :: I)) , ty :: Infer (ty :: InferTy :: FloatVar (_)) => Some (CastTy :: Float) , ty :: Uint (u) => Some (CastTy :: Int (IntTy :: U (u))) , ty :: Float (_) => Some (CastTy :: Float) , ty :: Adt (d , _) if d . is_enum () && d . is_payloadfree () => Some (CastTy :: Int (IntTy :: CEnum)) , ty :: RawPtr (ty , mutbl) => Some (CastTy :: Ptr (ty :: TypeAndMut { ty , mutbl })) , ty :: FnPtr (..) => Some (CastTy :: FnPtr) , _ => None , } } }}}

macro_rules! mir_cast_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_cast_kind in module {}", module_path!());
    };
}

mkfn!{
    mir_cast_kind_introspect!();
    # [doc = " Returns `mir::CastKind` from the given parameters."] pub fn mir_cast_kind < 'tcx > (from_ty : Ty < 'tcx > , cast_ty : Ty < 'tcx >) -> mir :: CastKind { let from = CastTy :: from_ty (from_ty) ; let cast = CastTy :: from_ty (cast_ty) ; let cast_kind = match (from , cast) { (Some (CastTy :: Ptr (_) | CastTy :: FnPtr) , Some (CastTy :: Int (_))) => { mir :: CastKind :: PointerExposeProvenance } (Some (CastTy :: Int (_)) , Some (CastTy :: Ptr (_))) => mir :: CastKind :: PointerWithExposedProvenance , (Some (CastTy :: Int (_)) , Some (CastTy :: Int (_))) => mir :: CastKind :: IntToInt , (Some (CastTy :: FnPtr) , Some (CastTy :: Ptr (_))) => mir :: CastKind :: FnPtrToPtr , (Some (CastTy :: Float) , Some (CastTy :: Int (_))) => mir :: CastKind :: FloatToInt , (Some (CastTy :: Int (_)) , Some (CastTy :: Float)) => mir :: CastKind :: IntToFloat , (Some (CastTy :: Float) , Some (CastTy :: Float)) => mir :: CastKind :: FloatToFloat , (Some (CastTy :: Ptr (_)) , Some (CastTy :: Ptr (_))) => mir :: CastKind :: PtrToPtr , (_ , _) => { bug ! ("Attempting to cast non-castable types {:?} and {:?}" , from_ty , cast_ty) } } ; cast_kind }
}