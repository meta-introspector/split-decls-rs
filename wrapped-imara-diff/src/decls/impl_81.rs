macro_rules! deps {
    () => {
        UnifiedDiffPrinter!();
        BasicLineDiffPrinter!();
        Token!();
        EndsWithNewline!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T : EndsWithNewline + Hash + Eq + Display + ? Sized > UnifiedDiffPrinter for BasicLineDiffPrinter < '_ , T > { fn display_header (& self , mut f : impl fmt :: Write , start_before : u32 , start_after : u32 , len_before : u32 , len_after : u32 ,) -> fmt :: Result { writeln ! (f , "@@ -{},{} +{},{} @@" , start_before + 1 , len_before , start_after + 1 , len_after) } fn display_context_token (& self , mut f : impl fmt :: Write , token : Token) -> fmt :: Result { write ! (f , " {}" , & self . 0 [token]) ? ; if ! & self . 0 [token] . ends_with_newline () { writeln ! (f) ? ; } Ok (()) } fn display_hunk (& self , mut f : impl fmt :: Write , before : & [Token] , after : & [Token] ,) -> fmt :: Result { if let Some (& last) = before . last () { for & token in before { let token = self . 0 [token] ; write ! (f , "-{token}") ? ; } if ! self . 0 [last] . ends_with_newline () { writeln ! (f) ? ; } } if let Some (& last) = after . last () { for & token in after { let token = self . 0 [token] ; write ! (f , "+{token}") ? ; } if ! self . 0 [last] . ends_with_newline () { writeln ! (f) ? ; } } Ok (()) } }
    };
}

impl_81!();