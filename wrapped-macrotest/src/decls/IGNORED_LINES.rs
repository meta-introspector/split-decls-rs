macro_rules! IGNORED_LINES {
    () => {
        const IGNORED_LINES : [& str ; 5] = ["#![feature(prelude_import)]" , "#[prelude_import]" , "use std::prelude::" , "#[macro_use]" , "extern crate std;" ,] ;
    };
}

IGNORED_LINES!();