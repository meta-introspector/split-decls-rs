// Generated macro for impl_215 (impl)
macro_rules! Depcrateimpl_215 {
() => {
// Module: crate
// Provides: {"impl_215"}
// Dependencies: {}
impl Terminator { # [doc = " Convert this to the csv_core type of the same name."] fn to_core (self) -> csv_core :: Terminator { match self { Terminator :: CRLF => csv_core :: Terminator :: CRLF , Terminator :: Any (b) => csv_core :: Terminator :: Any (b) , } } }
};
}
