macro_rules! char_property_functions {
    () => {
        macro_rules ! char_property_functions { { $ (mod $ module : ident ; static $ property_names : ident = [$ ($ prop : ident ,) *] ;) * } => { $ (property_functions ! ($ module , $ property_names , [$ ($ prop ,) *]) ;) * } ; { $ (mod $ module : ident ; static $ property_names : ident = [$ (($ _name : tt , $ prop : ident) ,) *] ;) * } => { $ (property_functions ! ($ module , $ property_names , [$ ($ prop ,) *]) ;) * } ; }
    };
}

char_property_functions!();