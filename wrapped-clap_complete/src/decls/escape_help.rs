macro_rules! escape_help {
    () => {
        # [doc = " Escape help string inside single quotes and brackets"] fn escape_help (string : & str) -> String { string . replace ('\\' , "\\\\") . replace ('\'' , "'\\''") . replace ('[' , "\\[") . replace (']' , "\\]") . replace (':' , "\\:") . replace ('$' , "\\$") . replace ('`' , "\\`") . replace ('\n' , " ") }
    };
}

escape_help!()