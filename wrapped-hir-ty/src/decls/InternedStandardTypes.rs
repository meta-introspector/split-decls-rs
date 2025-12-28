macro_rules! InternedStandardTypes {
    () => {
        # [derive (Debug , Clone)] struct InternedStandardTypes < 'db > { unit : Ty < 'db > , never : Ty < 'db > , char : Ty < 'db > , bool : Ty < 'db > , i8 : Ty < 'db > , i16 : Ty < 'db > , i32 : Ty < 'db > , i64 : Ty < 'db > , i128 : Ty < 'db > , isize : Ty < 'db > , u8 : Ty < 'db > , u16 : Ty < 'db > , u32 : Ty < 'db > , u64 : Ty < 'db > , u128 : Ty < 'db > , usize : Ty < 'db > , f16 : Ty < 'db > , f32 : Ty < 'db > , f64 : Ty < 'db > , f128 : Ty < 'db > , static_str_ref : Ty < 'db > , error : Ty < 'db > , re_static : Region < 'db > , re_error : Region < 'db > , re_erased : Region < 'db > , empty_args : GenericArgs < 'db > , }
    };
}

InternedStandardTypes!()