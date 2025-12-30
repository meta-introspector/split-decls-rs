// Generated macro for impl_26 (impl)
macro_rules! Depcrate_list_formatterimpl_26 {
() => {
// Module: crate::list_formatter
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a , W : Writeable + 'a , I : Iterator < Item = W > + Clone + 'a > core :: fmt :: Display for FormattedList < 'a , W , I > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . write_to (f) } }
};
}
