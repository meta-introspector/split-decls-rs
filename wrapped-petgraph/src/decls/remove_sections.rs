macro_rules! remove_sections {
    () => {
        macro_rules ! remove_sections { ([$ ($ stack : tt) *]) => { $ ($ stack) * } ; ([$ ($ stack : tt) *] { $ ($ tail : tt) * }) => { $ ($ stack) * { remove_sections_inner ! ([] $ ($ tail) *) ; } } ; ([$ ($ stack : tt) *] $ t : tt $ ($ tail : tt) *) => { remove_sections ! ([$ ($ stack) * $ t] $ ($ tail) *) ; } ; }
    };
}

remove_sections!()