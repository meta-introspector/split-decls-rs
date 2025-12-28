macro_rules! write_requires_scopes {
    () => {
        fn write_requires_scopes (sdl : & mut String , requires_scopes : & [String]) { write ! (sdl , " @requiresScopes(scopes: [{}])" , requires_scopes . iter () . map (| x | { "[" . to_string () + & x . split_whitespace () . map (| y | "\"" . to_string () + y + "\"") . collect ::< Vec < _ >> () . join (", ") + "]" }) . collect ::< Vec < _ >> () . join (", ")) . ok () ; }
    };
}

write_requires_scopes!();