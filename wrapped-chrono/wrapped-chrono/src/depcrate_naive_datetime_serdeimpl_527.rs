// Generated macro for impl_527 (impl)
macro_rules! Depcrate_naive_datetime_serdeimpl_527 {
() => {
// Module: crate::naive::datetime::serde
// Provides: {"impl_527"}
// Dependencies: {}
impl de :: Visitor < '_ > for NaiveDateTimeVisitor { type Value = NaiveDateTime ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a formatted date and time string") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { value . parse () . map_err (E :: custom) } }
};
}
