macro_rules! deps {
    () => {
        From!();
        Field!();
    };
}

macro_rules! from_initializer {
    () => {
        deps!();
        fn from_initializer (from_field : & Field , backtrace_field : Option < & Field > , source_var : & Ident ,) -> TokenStream { let from_member = & from_field . member ; let some_source = if type_is_option (from_field . ty) { quote ! (:: core :: option :: Option :: Some (# source_var)) } else { quote ! (# source_var) } ; let backtrace = backtrace_field . map (| backtrace_field | { let backtrace_member = & backtrace_field . member ; if type_is_option (backtrace_field . ty) { quote ! { # backtrace_member : :: core :: option :: Option :: Some (:: thiserror ::# private :: Backtrace :: capture ()) , } } else { quote ! { # backtrace_member : :: core :: convert :: From :: from (:: thiserror ::# private :: Backtrace :: capture ()) , } } }) ; quote ! ({ # from_member : # some_source , # backtrace }) }
    };
}

from_initializer!();