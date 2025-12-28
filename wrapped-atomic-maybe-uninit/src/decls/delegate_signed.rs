macro_rules! delegate_signed {
    () => {
        # [allow (unused_macros)] macro_rules ! delegate_signed { ($ delegate : ident , u8) => { $ delegate ! (i8 , u8) ; } ; ($ delegate : ident , u16) => { $ delegate ! (i16 , u16) ; } ; ($ delegate : ident , u32) => { $ delegate ! (i32 , u32) ; } ; ($ delegate : ident , u64) => { $ delegate ! (i64 , u64) ; } ; ($ delegate : ident , u128) => { $ delegate ! (i128 , u128) ; } ; }
    };
}

delegate_signed!()