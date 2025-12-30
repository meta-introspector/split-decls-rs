// Generated macro for parse_dialect (function)
macro_rules! Depcrate_attributes_prototypeparse_dialect {
() => {
// Module: crate::attributes::prototype
// Provides: {"parse_dialect"}
// Dependencies: {}
fn parse_dialect < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , dialect : Option < (Symbol , Span) > , failed : & mut bool ,) -> Option < (MirDialect , Span) > { let (dialect , span) = dialect ? ; let dialect = match dialect { sym :: analysis => MirDialect :: Analysis , sym :: built => MirDialect :: Built , sym :: runtime => MirDialect :: Runtime , _ => { cx . expected_specific_argument (span , & [sym :: analysis , sym :: built , sym :: runtime]) ; * failed = true ; return None ; } } ; Some ((dialect , span)) }
};
}
