macro_rules! is_printable_bytes {
    () => {
        # [inline] fn is_printable_bytes (action : Action , byte : u8) -> bool { const DEL : u8 = 0x7f ; (action == Action :: Print && byte != DEL) || action == Action :: BeginUtf8 || (action == Action :: Execute && byte . is_ascii_whitespace ()) }
    };
}

is_printable_bytes!()