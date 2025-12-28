macro_rules! deps {
    () => {
        BorrowDecode!();
        Decode!();
        DecodeError!();
        BorrowDecoder!();
    };
}

macro_rules! impl_borrow_decode_with_context {
    () => {
        deps!();
        # [doc = " Helper macro to implement `BorrowDecode` for any type that implements `Decode`."] # [macro_export] macro_rules ! impl_borrow_decode_with_context { ($ ty : ty , $ context : ty $ (, $ param : tt) *) => { impl <'de $ (, $ param) *> $ crate :: BorrowDecode <'de , $ context > for $ ty { fn borrow_decode < D : $ crate :: de :: BorrowDecoder <'de , Context = $ context >> (decoder : & mut D ,) -> core :: result :: Result < Self , $ crate :: error :: DecodeError > { $ crate :: Decode :: decode (decoder) } } } ; }
    };
}

impl_borrow_decode_with_context!()