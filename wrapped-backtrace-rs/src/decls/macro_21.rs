macro_rules! macro_21 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (feature = "cpp_demangle")] { struct OptionCppSymbol <'a > (Option <:: cpp_demangle :: BorrowedSymbol <'a >>) ; impl <'a > OptionCppSymbol <'a > { fn parse (input : &'a [u8]) -> OptionCppSymbol <'a > { OptionCppSymbol (:: cpp_demangle :: BorrowedSymbol :: new (input) . ok ()) } fn none () -> OptionCppSymbol <'a > { OptionCppSymbol (None) } } } }
    };
}

macro_21!()