// Generated macro for FormattedList (struct)
macro_rules! Depcrate_list_formatterFormattedList {
() => {
// Module: crate::list_formatter
// Provides: {"FormattedList"}
// Dependencies: {}
# [doc = " The [`Writeable`] implementation that is returned by [`ListFormatter::format`]. See"] # [doc = " the [`writeable`] crate for how to consume this."] # [derive (Debug)] pub struct FormattedList < 'a , W : Writeable + 'a , I : Iterator < Item = W > + Clone + 'a > { formatter : & 'a ListFormatter , values : I , }
};
}
