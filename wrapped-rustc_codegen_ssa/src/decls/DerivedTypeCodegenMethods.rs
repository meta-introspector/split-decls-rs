macro_rules! deps {
    () => {
        MiscCodegenMethods!();
        BaseTypeCodegenMethods!();
    };
}

macro_rules! DerivedTypeCodegenMethods {
    () => {
        deps!();
        pub trait DerivedTypeCodegenMethods < 'tcx > : BaseTypeCodegenMethods + MiscCodegenMethods < 'tcx > + HasTyCtxt < 'tcx > + HasTypingEnv < 'tcx > { fn type_int (& self) -> Self :: Type { match & self . sess () . target . c_int_width { 16 => self . type_i16 () , 32 => self . type_i32 () , 64 => self . type_i64 () , width => bug ! ("Unsupported c_int_width: {}" , width) , } } fn type_from_integer (& self , i : Integer) -> Self :: Type { use Integer :: * ; match i { I8 => self . type_i8 () , I16 => self . type_i16 () , I32 => self . type_i32 () , I64 => self . type_i64 () , I128 => self . type_i128 () , } } fn type_from_float (& self , f : Float) -> Self :: Type { use Float :: * ; match f { F16 => self . type_f16 () , F32 => self . type_f32 () , F64 => self . type_f64 () , F128 => self . type_f128 () , } } fn type_needs_drop (& self , ty : Ty < 'tcx >) -> bool { ty . needs_drop (self . tcx () , self . typing_env ()) } fn type_is_sized (& self , ty : Ty < 'tcx >) -> bool { ty . is_sized (self . tcx () , self . typing_env ()) } fn type_is_freeze (& self , ty : Ty < 'tcx >) -> bool { ty . is_freeze (self . tcx () , self . typing_env ()) } fn type_from_primitive (& self , p : Primitive) -> Self :: Type { use Primitive :: * ; match p { Int (i , _) => self . type_from_integer (i) , Float (f) => self . type_from_float (f) , Pointer (address_space) => self . type_ptr_ext (address_space) , } } fn type_from_scalar (& self , s : Scalar) -> Self :: Type { self . type_from_primitive (s . primitive ()) } }
    };
}

DerivedTypeCodegenMethods!()