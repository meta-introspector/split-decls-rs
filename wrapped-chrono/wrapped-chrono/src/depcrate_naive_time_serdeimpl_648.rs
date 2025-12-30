// Generated macro for impl_648 (impl)
macro_rules! Depcrate_naive_time_serdeimpl_648 {
() => {
// Module: crate::naive::time::serde
// Provides: {"impl_648"}
// Dependencies: {}
impl de :: Visitor < '_ > for NaiveTimeVisitor { type Value = NaiveTime ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a formatted time string") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { value . parse () . map_err (E :: custom) } }
};
}
