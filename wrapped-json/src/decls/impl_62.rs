macro_rules! deps {
    () => {
        Error!();
        ErrorCode!();
        ErrorImpl!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Error { # [cold] pub (crate) fn syntax (code : ErrorCode , line : usize , column : usize) -> Self { Error { err : Box :: new (ErrorImpl { code , line , column }) , } } # [doc (hidden)] # [cold] pub fn io (error : io :: Error) -> Self { Error { err : Box :: new (ErrorImpl { code : ErrorCode :: Io (error) , line : 0 , column : 0 , }) , } } # [cold] pub (crate) fn fix_position < F > (self , f : F) -> Self where F : FnOnce (ErrorCode) -> Error , { if self . err . line == 0 { f (self . err . code) } else { self } } }
    };
}

impl_62!()