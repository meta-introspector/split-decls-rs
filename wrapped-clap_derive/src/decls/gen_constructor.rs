macro_rules! deps {
    () => {
        Kind!();
        Item!();
        Ty!();
    };
}

macro_rules! gen_constructor {
    () => {
        deps!();
        pub (crate) fn gen_constructor (fields : & [(& Field , Item)]) -> Result < TokenStream , syn :: Error > { let fields = fields . iter () . map (| (field , item) | { let field_name = field . ident . as_ref () . unwrap () ; let kind = item . kind () ; let arg_matches = format_ident ! ("__clap_arg_matches") ; let genned = match & * kind { Kind :: Command (_) | Kind :: Value | Kind :: ExternalSubcommand => { abort ! { kind . span () , "`{}` cannot be used with `arg`" , kind . name () , } } Kind :: Subcommand (ty) => { let subcmd_type = match (* * ty , sub_type (& field . ty)) { (Ty :: Option , Some (sub_type)) => sub_type , _ => & field . ty , } ; match * * ty { Ty :: Option => { quote_spanned ! { kind . span () => # field_name : { if # arg_matches . subcommand_name () . map (<# subcmd_type as clap :: Subcommand >:: has_subcommand) . unwrap_or (false) { Some (<# subcmd_type as clap :: FromArgMatches >:: from_arg_matches_mut (# arg_matches) ?) } else { None } } } } , Ty :: Other => { quote_spanned ! { kind . span () => # field_name : { <# subcmd_type as clap :: FromArgMatches >:: from_arg_matches_mut (# arg_matches) ? } } } , Ty :: Unit | Ty :: Vec | Ty :: OptionOption | Ty :: OptionVec | Ty :: VecVec | Ty :: OptionVecVec => { abort ! (ty . span () , "{} types are not supported for subcommand" , ty . as_str ()) ; } } } Kind :: Flatten (ty) => { let inner_type = match (* * ty , sub_type (& field . ty)) { (Ty :: Option , Some (sub_type)) => sub_type , _ => & field . ty , } ; match * * ty { Ty :: Other => { quote_spanned ! { kind . span () => # field_name : <# inner_type as clap :: FromArgMatches >:: from_arg_matches_mut (# arg_matches) ? } } , Ty :: Option => { quote_spanned ! { kind . span () => # field_name : { let group_id = <# inner_type as clap :: Args >:: group_id () . expect ("asserted during `Arg` creation") ; if # arg_matches . contains_id (group_id . as_str ()) { Some (<# inner_type as clap :: FromArgMatches >:: from_arg_matches_mut (# arg_matches) ?) } else { None } } } } , Ty :: Unit | Ty :: Vec | Ty :: OptionOption | Ty :: OptionVec | Ty :: VecVec | Ty :: OptionVecVec => { abort ! (ty . span () , "{} types are not supported for flatten" , ty . as_str ()) ; } } } , Kind :: Skip (val , _) => match val { None => quote_spanned ! (kind . span () => # field_name : Default :: default ()) , Some (val) => quote_spanned ! (kind . span () => # field_name : (# val) . into ()) , } , Kind :: Arg (ty) | Kind :: FromGlobal (ty) => { gen_parsers (item , ty , field_name , field , None) ? } } ; Ok (genned) }) . collect :: < Result < Vec < _ > , syn :: Error > > () ? ; Ok (quote ! { { # (# fields) ,* } }) }
    };
}

gen_constructor!();