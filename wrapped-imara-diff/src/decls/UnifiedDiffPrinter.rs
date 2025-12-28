macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! UnifiedDiffPrinter {
    () => {
        deps!();
        pub trait UnifiedDiffPrinter { fn display_header (& self , f : impl fmt :: Write , start_before : u32 , start_after : u32 , len_before : u32 , len_after : u32 ,) -> fmt :: Result ; fn display_context_token (& self , f : impl fmt :: Write , token : Token) -> fmt :: Result ; fn display_hunk (& self , f : impl fmt :: Write , before : & [Token] , after : & [Token]) -> fmt :: Result ; }
    };
}

UnifiedDiffPrinter!();