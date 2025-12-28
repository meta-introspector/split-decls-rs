macro_rules! deps {
    () => {
        AllowedEnumVariants!();
        DecodeError!();
        BorrowDecoder!();
        BorrowDecode!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        impl < 'de , T , U , Context > BorrowDecode < 'de , Context > for Result < T , U > where T : BorrowDecode < 'de , Context > , U : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let is_ok = u32 :: decode (decoder) ? ; match is_ok { 0 => { let t = T :: borrow_decode (decoder) ? ; Ok (Ok (t)) } 1 => { let u = U :: borrow_decode (decoder) ? ; Ok (Err (u)) } x => Err (DecodeError :: UnexpectedVariant { found : x , allowed : & crate :: error :: AllowedEnumVariants :: Range { max : 1 , min : 0 } , type_name : core :: any :: type_name :: < Result < T , U > > () , }) , } } }
    };
}

impl_376!();