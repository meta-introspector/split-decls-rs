// Generated macro for impl_201 (impl)
macro_rules! Depcrate_neoimpl_201 {
() => {
// Module: crate::neo
// Provides: {"impl_201"}
// Dependencies: {}
impl Writeable for FormattedDateTime < '_ > { fn write_to_parts < S : writeable :: PartsWrite + ? Sized > (& self , sink : & mut S ,) -> Result < () , fmt :: Error > { let result = try_write_pattern_items (self . pattern . metadata () , self . pattern . iter_items () , & self . input , & self . names , self . names . decimal_formatter , sink ,) ; match result { Ok (Ok (())) => Ok (()) , Err (fmt :: Error) => Err (fmt :: Error) , Ok (Err (e)) => { debug_assert ! (false , "unexpected error in FormattedDateTime: {e:?}") ; Ok (()) } } } }
};
}
