macro_rules! deps {
    () => {
        Action!();
        InputWrapper!();
    };
}

macro_rules! read_byte {
    () => {
        deps!();
        # [doc = " Try to read one byte from `in_iter` and call `f` with the read byte as an argument,"] # [doc = " returning the result."] # [doc = " If reading fails, `Action::End is returned`"] # [inline] fn read_byte < F > (in_iter : & mut InputWrapper , flags : u32 , f : F) -> Action where F : FnOnce (u8) -> Action , { match in_iter . read_byte () { None => end_of_input (flags) , Some (byte) => f (byte) , } }
    };
}

read_byte!()