// Generated macro for macro_125 (macro)
macro_rules! Depcratemacro_125 {
() => {
// Module: crate
// Provides: {"macro_125"}
// Dependencies: {}
simple_newtypes ! { # [doc = " Stated virtual memory address."] # [doc = ""] # [doc = " See the module documentation for details."] type Svma = usize where default = 0 , display = "{:#x}" ; # [doc = " Actual virtual memory address."] # [doc = ""] # [doc = " See the module documentation for details."] type Avma = usize where default = 0 , display = "{:#x}" ; # [doc = " Virtual memory bias."] # [doc = ""] # [doc = " See the module documentation for details."] type Bias = usize where default = 0 , display = "{:#x}" ; }
};
}
