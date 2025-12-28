macro_rules! escape_value {
    () => {
        # [doc = " Escape value string inside single quotes and parentheses"] fn escape_value (string : & str) -> String { string . replace ('\\' , "\\\\") . replace ('\'' , "'\\''") . replace ('[' , "\\[") . replace (']' , "\\]") . replace (':' , "\\:") . replace ('$' , "\\$") . replace ('`' , "\\`") . replace ('(' , "\\(") . replace (')' , "\\)") . replace (' ' , "\\ ") }
    };
}

escape_value!();