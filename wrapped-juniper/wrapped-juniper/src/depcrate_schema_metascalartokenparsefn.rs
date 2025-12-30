// Generated macro for ScalarTokenParseFn (type)
macro_rules! Depcrate_schema_metaScalarTokenParseFn {
() => {
// Module: crate::schema::meta
// Provides: {"ScalarTokenParseFn"}
// Dependencies: {}
# [doc = " Shortcut for a [`ScalarToken`] parsing function."] pub type ScalarTokenParseFn < S > = for < 'b > fn (ScalarToken < 'b >) -> Result < S , ParseError > ;
};
}
