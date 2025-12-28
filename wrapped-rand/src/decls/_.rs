macro_rules! _ {
    () => {
        const _ : () = unsafe { impl_fill ! (i8 , i16 , i32 , i64 , i128 ,) } ;
    };
}

_!()