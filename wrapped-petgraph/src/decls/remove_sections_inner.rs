macro_rules! remove_sections_inner {
    () => {
        macro_rules ! remove_sections_inner { ([$ ($ stack : tt) *]) => { $ ($ stack) * } ; ([$ ($ stack : tt) *] @ escape $ _x : tt $ ($ t : tt) *) => { remove_sections_inner ! ([$ ($ stack) *] $ ($ t) *) ; } ; ([$ ($ stack : tt) *] @ section $ x : ident $ ($ t : tt) *) => { remove_sections_inner ! ([$ ($ stack) *] $ ($ t) *) ; } ; ([$ ($ stack : tt) *] $ t : tt $ ($ tail : tt) *) => { remove_sections_inner ! ([$ ($ stack) * $ t] $ ($ tail) *) ; } ; }
    };
}

remove_sections_inner!();