// Generated macro for value_parser_tests (module)
macro_rules! Depcrate_sharedvalue_parser_tests {
() => {
// Module: crate::shared
// Provides: {"value_parser_tests"}
// Dependencies: {}
# [cfg (test)] mod value_parser_tests { use clap :: Parser ; use super :: { AsRange , AsTime , ParseRenameFraction } ; # [test] fn rename_fraction () { # [derive (Debug , clap :: Parser)] pub struct Cmd { # [clap (long , short ='a' , value_parser = ParseRenameFraction)] pub arg : Option < Option < f32 > > , } let c = Cmd :: parse_from (["cmd" , "-a"]) ; assert_eq ! (c . arg , Some (None) , "this means we need to fill in the default") ; let c = Cmd :: parse_from (["cmd" , "-a=50%"]) ; assert_eq ! (c . arg , Some (Some (0.5)) , "percentages become a fraction") ; let c = Cmd :: parse_from (["cmd" , "-a=100%"]) ; assert_eq ! (c . arg , Some (Some (1.0))) ; let c = Cmd :: parse_from (["cmd" , "-a=5"]) ; assert_eq ! (c . arg , Some (Some (0.5)) , "another way to specify fractions") ; let c = Cmd :: parse_from (["cmd" , "-a=75"]) ; assert_eq ! (c . arg , Some (Some (0.75))) ; } # [test] fn range () { # [derive (Debug , clap :: Parser)] pub struct Cmd { # [clap (long , short ='l' , value_parser = AsRange)] pub arg : Option < std :: ops :: RangeInclusive < u32 > > , } let c = Cmd :: parse_from (["cmd" , "-l=1,10"]) ; assert_eq ! (c . arg , Some (1 ..= 10)) ; } # [test] fn since () { # [derive (Debug , clap :: Parser)] pub struct Cmd { # [clap (long , long = "since" , value_parser = AsTime)] pub arg : Option < gix :: date :: Time > , } let c = Cmd :: parse_from (["cmd" , "--since" , "2 weeks ago"]) ; assert ! (matches ! (c . arg , Some (gix :: date :: Time { .. }))) ; } }
};
}
