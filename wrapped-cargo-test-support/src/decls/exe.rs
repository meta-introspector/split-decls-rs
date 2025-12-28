macro_rules! exe {
    () => {
        # [doc = " `$name$EXE`"] pub fn exe (name : & str) -> String { format ! ("{}{}" , name , EXE_SUFFIX) }
    };
}

exe!();