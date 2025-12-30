// Generated macro for InputValueParseFn (type)
macro_rules! Depcrate_schema_metaInputValueParseFn {
() => {
// Module: crate::schema::meta
// Provides: {"InputValueParseFn"}
// Dependencies: {}
# [doc = " Shortcut for an [`InputValue`] parsing function."] pub type InputValueParseFn < S > = for < 'b > fn (& 'b InputValue < S >) -> Result < () , FieldError < S > > ;
};
}
