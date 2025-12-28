macro_rules! macro_294 {
    () => {
        define_vocabulary ! { # [doc = " The `<substitution>` variants that are encoded directly in the grammar,"] # [doc = " rather than as back references to other components in the substitution"] # [doc = " table."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum WellKnownComponent { Std (b"St" , "std") , StdAllocator (b"Sa" , "std::allocator") , StdString1 (b"Sb" , "std::basic_string") , StdString2 (b"Ss" , "std::string") , StdIstream (b"Si" , "std::basic_istream<char, std::char_traits<char> >") , StdOstream (b"So" , "std::ostream") , StdIostream (b"Sd" , "std::basic_iostream<char, std::char_traits<char> >") } }
    };
}

macro_294!()