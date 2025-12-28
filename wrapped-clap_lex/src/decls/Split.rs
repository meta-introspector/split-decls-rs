macro_rules! Split {
    () => {
        pub struct Split < 's , 'n > { haystack : Option < & 's OsStr > , needle : & 'n str , }
    };
}

Split!();