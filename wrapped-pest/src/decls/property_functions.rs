macro_rules! property_functions {
    () => {
        macro_rules ! property_functions { ($ module : ident , $ property_names : ident , [$ ($ prop : ident ,) *]) => { # [allow (unused)] mod $ module ; $ (pub fn $ prop (c : char) -> bool { self ::$ module ::$ prop . contains_char (c) }) * pub static $ property_names : & [& str] = & [$ (stringify ! ($ prop) ,) *] ; } ; }
    };
}

property_functions!();