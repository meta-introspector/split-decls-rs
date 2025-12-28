macro_rules! deps {
    () => {
        BorrowDecode!();
        AllowedEnumVariants!();
        BorrowDecoder!();
        DecodeError!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Bound < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { match u32 :: decode (decoder) ? { 0 => Ok (Bound :: Unbounded) , 1 => Ok (Bound :: Included (T :: borrow_decode (decoder) ?)) , 2 => Ok (Bound :: Excluded (T :: borrow_decode (decoder) ?)) , x => Err (DecodeError :: UnexpectedVariant { allowed : & crate :: error :: AllowedEnumVariants :: Range { max : 2 , min : 0 } , found : x , type_name : core :: any :: type_name :: < Bound < T > > () , }) , } } }
    };
}

impl_388!();