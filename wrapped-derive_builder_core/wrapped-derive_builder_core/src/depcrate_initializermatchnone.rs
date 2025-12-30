// Generated macro for MatchNone (enum)
macro_rules! Depcrate_initializerMatchNone {
() => {
// Module: crate::initializer
// Provides: {"MatchNone"}
// Dependencies: {}
# [doc = " To be used inside of `#struct_field: match self.#builder_field { ... }`"] enum MatchNone < 'a > { # [doc = " Inner value must be a valid Rust expression"] DefaultTo { expr : & 'a DefaultExpression , crate_root : & 'a syn :: Path , } , # [doc = " Inner value must be the field identifier"] # [doc = ""] # [doc = " The default struct must be in scope in the build_method."] UseDefaultStructField (& 'a syn :: Ident) , # [doc = " Inner value must be the field name"] ReturnError { crate_root : & 'a syn :: Path , field_name : String , span : Option < Span > , } , }
};
}
