macro_rules! Error {
    () => {
        # [doc = " A structure to represent errors coming out of libgit2."] # [derive (Debug , PartialEq)] pub struct Error { code : c_int , klass : c_int , message : Box < str > , }
    };
}

Error!()