// Generated macro for impl_59 (impl)
macro_rules! Depcrate_hyperlinkimpl_59 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_59"}
// Dependencies: {}
impl std :: fmt :: Display for HyperlinkFormatError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use self :: HyperlinkFormatErrorKind :: * ; match self . kind { NoVariables => { let mut aliases = hyperlink_aliases () ; aliases . sort_by_key (| alias | { alias . display_priority () . unwrap_or (i16 :: MAX) }) ; let names : Vec < & str > = aliases . iter () . map (| alias | alias . name ()) . collect () ; write ! (f , "at least a {{path}} variable is required in a \
                     hyperlink format, or otherwise use a valid alias: \
                     {aliases}" , aliases = names . join (", ") ,) } NoPathVariable => { write ! (f , "the {{path}} variable is required in a hyperlink format" ,) } NoLineVariable => { write ! (f , "the hyperlink format contains a {{column}} variable, \
                     but no {{line}} variable is present" ,) } InvalidVariable (ref name) => { write ! (f , "invalid hyperlink format variable: '{name}', choose \
                     from: path, line, column, host, wslprefix" ,) } InvalidScheme => { write ! (f , "the hyperlink format must start with a valid URL scheme, \
                     i.e., [0-9A-Za-z+-.]+:" ,) } InvalidCloseVariable => { write ! (f , "unopened variable: found '}}' without a \
                     corresponding '{{' preceding it" ,) } UnclosedVariable => { write ! (f , "unclosed variable: found '{{' without a \
                     corresponding '}}' following it" ,) } } } }
};
}
