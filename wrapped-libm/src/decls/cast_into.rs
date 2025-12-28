macro_rules! deps {
    () => {
        CastInto!();
    };
}

macro_rules! cast_into {
    () => {
        deps!();
        macro_rules ! cast_into { ($ ty : ty) => { cast_into ! ($ ty ; usize , isize , u8 , i8 , u16 , i16 , u32 , i32 , u64 , i64 , u128 , i128) ; } ; ($ ty : ty ; $ ($ into : ty) ,*) => { $ (impl CastInto <$ into > for $ ty { fn cast (self) -> $ into { # [cfg (not (feature = "compiler-builtins"))] debug_assert ! (<$ into >:: try_from (self) . is_ok () , "failed cast from {self}") ; self as $ into } fn cast_lossy (self) -> $ into { self as $ into } }) * } ; }
    };
}

cast_into!();