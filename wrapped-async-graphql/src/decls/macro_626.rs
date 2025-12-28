macro_rules! macro_626 {
    () => {
        pin_project ! { pub (crate) struct ReaderStream < T > { buf : [u8 ; 2048] , # [pin] reader : T , } }
    };
}

macro_626!();