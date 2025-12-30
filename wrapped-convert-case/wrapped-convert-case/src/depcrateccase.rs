// Generated macro for ccase (macro)
macro_rules! Depcrateccase {
() => {
// Module: crate
// Provides: {"ccase"}
// Dependencies: {}
# [doc = " Convert an identifier into a case."] # [doc = ""] # [doc = " The macro can be used as follows."] # [doc = " ```"] # [doc = " use convert_case::ccase;"] # [doc = ""] # [doc = " assert_eq!(ccase!(snake, \"myVarName\"), \"my_var_name\");"] # [doc = " // equivalent to"] # [doc = " // \"myVarName\".to_case(Case::Snake)"] # [doc = " ```"] # [doc = " You can also specify a _from_ case, or the case that determines how the input"] # [doc = " string is split into words."] # [doc = " ```"] # [doc = " use convert_case::ccase;"] # [doc = ""] # [doc = " assert_eq!(ccase!(sentence -> snake, \"Ice-cream sales\"), \"ice-cream_sales\");"] # [doc = " // equivalent to"] # [doc = " // \"Ice-cream sales\".from_case(Case::Sentence).to_case(Case::Snake)"] # [doc = " ```"] # [macro_export] macro_rules ! ccase { ($ case : ident , $ e : expr) => { convert_case :: Converter :: new () . to_case (convert_case :: case ! ($ case)) . convert ($ e) } ; ($ from : ident -> $ to : ident , $ e : expr) => { convert_case :: Converter :: new () . from_case (convert_case :: case ! ($ from)) . to_case (convert_case :: case ! ($ to)) . convert ($ e) } ; }
};
}
