macro_rules! SearchTestOwned {
    () => {
        struct SearchTestOwned { offset : usize , name : String , patterns : Vec < String > , haystack : String , matches : Vec < (usize , usize , usize) > , }
    };
}

SearchTestOwned!();