macro_rules! deps {
    () => {
        BorrowDecode!();
        DecodeError!();
        BorrowDecoder!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Option < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { match super :: decode_option_variant (decoder , core :: any :: type_name :: < Option < T > > ()) ? { Some (_) => { let val = T :: borrow_decode (decoder) ? ; Ok (Some (val)) } None => Ok (None) , } } }
    };
}

impl_374!()