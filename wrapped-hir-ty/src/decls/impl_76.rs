macro_rules! deps {
    () => {
        InternedStandardTypes!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'db > InternedStandardTypes < 'db > { fn new (interner : DbInterner < 'db >) -> Self { let str = Ty :: new (interner , rustc_type_ir :: TyKind :: Str) ; let re_static = Region :: new_static (interner) ; Self { unit : Ty :: new_unit (interner) , never : Ty :: new (interner , TyKind :: Never) , char : Ty :: new (interner , TyKind :: Char) , bool : Ty :: new (interner , TyKind :: Bool) , i8 : Ty :: new_int (interner , rustc_type_ir :: IntTy :: I8) , i16 : Ty :: new_int (interner , rustc_type_ir :: IntTy :: I16) , i32 : Ty :: new_int (interner , rustc_type_ir :: IntTy :: I32) , i64 : Ty :: new_int (interner , rustc_type_ir :: IntTy :: I64) , i128 : Ty :: new_int (interner , rustc_type_ir :: IntTy :: I128) , isize : Ty :: new_int (interner , rustc_type_ir :: IntTy :: Isize) , u8 : Ty :: new_uint (interner , rustc_type_ir :: UintTy :: U8) , u16 : Ty :: new_uint (interner , rustc_type_ir :: UintTy :: U16) , u32 : Ty :: new_uint (interner , rustc_type_ir :: UintTy :: U32) , u64 : Ty :: new_uint (interner , rustc_type_ir :: UintTy :: U64) , u128 : Ty :: new_uint (interner , rustc_type_ir :: UintTy :: U128) , usize : Ty :: new_uint (interner , rustc_type_ir :: UintTy :: Usize) , f16 : Ty :: new_float (interner , rustc_type_ir :: FloatTy :: F16) , f32 : Ty :: new_float (interner , rustc_type_ir :: FloatTy :: F32) , f64 : Ty :: new_float (interner , rustc_type_ir :: FloatTy :: F64) , f128 : Ty :: new_float (interner , rustc_type_ir :: FloatTy :: F128) , static_str_ref : Ty :: new_ref (interner , re_static , str , Mutability :: Not) , error : Ty :: new_error (interner , ErrorGuaranteed) , re_static , re_error : Region :: error (interner) , re_erased : Region :: new_erased (interner) , empty_args : GenericArgs :: new_from_iter (interner , []) , } } }
    };
}

impl_76!();