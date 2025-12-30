// Generated macro for macro_34 (macro)
macro_rules! Depcrate_symbolizemacro_34 {
() => {
// Module: crate::symbolize
// Provides: {"macro_34"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (feature = "cpp_demangle")] { struct OptionCppSymbol <'a > (Option <:: cpp_demangle :: BorrowedSymbol <'a >>) ; impl <'a > OptionCppSymbol <'a > { fn parse (input : &'a [u8]) -> OptionCppSymbol <'a > { OptionCppSymbol (:: cpp_demangle :: BorrowedSymbol :: new (input) . ok ()) } fn none () -> OptionCppSymbol <'a > { OptionCppSymbol (None) } } } }
};
}
