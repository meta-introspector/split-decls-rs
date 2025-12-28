macro_rules! deps {
    () => {
        SliceRead!();
    };
}

macro_rules! StrRead {
    () => {
        deps!();
        # [doc = " JSON input source that reads from a UTF-8 string."] pub struct StrRead < 'a > { delegate : SliceRead < 'a > , # [cfg (feature = "raw_value")] data : & 'a str , }
    };
}

StrRead!();