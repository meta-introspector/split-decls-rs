macro_rules! macro_258 {
    () => {
        pin_project ! { # [doc = " Stream for the [`AsyncBufReadExt::lines()`] method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Lines < R > { # [pin] reader : R , buf : String , bytes : Vec < u8 >, read : usize , } }
    };
}

macro_258!()