// Generated macro for impl_85 (impl)
macro_rules! Depcrate_fs_feature_xattrimpl_85 {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"impl_85"}
// Dependencies: {}
impl Display for Attribute { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_fmt (format_args ! ("{}: " , self . name)) ? ; if let Some (value) = custom_attr_display (self) { f . write_fmt (format_args ! ("<{value}>")) } else { match & self . value { None => f . write_str ("<empty>") , Some (value) => { if let Some (val) = custom_value_display (value) { f . write_fmt (format_args ! ("<{val}>")) } else if let Ok (v) = str :: from_utf8 (value) { f . write_fmt (format_args ! ("{:?}" , v . trim_end_matches (char :: from (0)))) } else if value . len () <= ATTRIBUTE_VALUE_MAX_HEX_LENGTH { f . write_fmt (format_args ! ("{value:02x?}")) } else { f . write_fmt (format_args ! ("<length {}>" , value . len ())) } } } } } }
};
}
