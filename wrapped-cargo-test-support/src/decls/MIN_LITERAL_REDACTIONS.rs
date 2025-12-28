macro_rules! MIN_LITERAL_REDACTIONS {
    () => {
        static MIN_LITERAL_REDACTIONS : & [(& str , & str)] = & [("[EXE]" , std :: env :: consts :: EXE_SUFFIX) , ("[BROKEN_PIPE]" , "Broken pipe (os error 32)") , ("[BROKEN_PIPE]" , "The pipe is being closed. (os error 232)") , ("[NOT_FOUND]" , "No such file or directory (os error 2)") , ("[NOT_FOUND]" , "The system cannot find the file specified. (os error 2)" ,) , ("[NOT_FOUND]" , "The system cannot find the path specified. (os error 3)" ,) , ("[NOT_FOUND]" , "Access is denied. (os error 5)") , ("[NOT_FOUND]" , "program not found") , ("[EXIT_STATUS]" , "exit status") , ("[EXIT_STATUS]" , "exit code") ,] ;
    };
}

MIN_LITERAL_REDACTIONS!()