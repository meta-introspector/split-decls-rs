// Generated macro for gen_has_subcommand (function)
macro_rules! Depcrate_derives_subcommandgen_has_subcommand {
() => {
// Module: crate::derives::subcommand
// Provides: {"gen_has_subcommand"}
// Dependencies: {}
fn gen_has_subcommand (variants : & [(& Variant , Item)]) -> Result < TokenStream , syn :: Error > { use syn :: Fields :: Unnamed ; let mut ext_subcmd = false ; let (flatten_variants , variants) : (Vec < _ > , Vec < _ >) = variants . iter () . filter_map (| (variant , item) | { let kind = item . kind () ; match & * kind { Kind :: Skip (_ , _) | Kind :: Arg (_) | Kind :: FromGlobal (_) | Kind :: Value => None , Kind :: ExternalSubcommand => { ext_subcmd = true ; None } Kind :: Flatten (_) | Kind :: Subcommand (_) | Kind :: Command (_) => Some ((variant , item)) , } }) . partition (| (_ , item) | { let kind = item . kind () ; matches ! (&* kind , Kind :: Flatten (_)) }) ; let subcommands = variants . iter () . map (| (_variant , item) | { let sub_name = item . cased_name () ; quote ! { if # sub_name == __clap_name { return true } } }) ; let child_subcommands = flatten_variants . iter () . map (| (variant , _attrs) | match variant . fields { Unnamed (ref fields) if fields . unnamed . len () == 1 => { let ty = & fields . unnamed [0] . ty ; Ok (quote ! { if <# ty as clap :: Subcommand >:: has_subcommand (__clap_name) { return true ; } }) } _ => abort ! (variant , "`flatten` is usable only with single-typed tuple variants") , }) . collect :: < Result < Vec < _ > , syn :: Error > > () ? ; let genned = if ext_subcmd { quote ! { true } } else { quote ! { # (# subcommands) * # (# child_subcommands) else * false } } ; Ok (genned) }
};
}
