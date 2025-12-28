macro_rules! macro_261 {
    () => {
        pin_project ! { # [doc = " Stream for the [`AsyncBufReadExt::split()`] method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Split < R > { # [pin] reader : R , buf : Vec < u8 >, read : usize , delim : u8 , } }
    };
}

macro_261!()