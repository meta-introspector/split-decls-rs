macro_rules! deps {
    () => {
        Item!();
        Kind!();
        Ty!();
    };
}

macro_rules! gen_updater {
    () => {
        deps!();
        pub (crate) fn gen_updater (fields : & [(& Field , Item)] , use_self : bool ,) -> Result < TokenStream , syn :: Error > { let mut genned_fields = Vec :: new () ; for (field , item) in fields { let field_name = field . ident . as_ref () . unwrap () ; let kind = item . kind () ; let access = if use_self { quote ! { # [allow (non_snake_case)] let # field_name = & mut self .# field_name ; } } else { quote ! () } ; let arg_matches = format_ident ! ("__clap_arg_matches") ; let genned = match & * kind { Kind :: Command (_) | Kind :: Value | Kind :: ExternalSubcommand => { abort ! { kind . span () , "`{}` cannot be used with `arg`" , kind . name () , } } Kind :: Subcommand (ty) => { let subcmd_type = match (* * ty , sub_type (& field . ty)) { (Ty :: Option , Some (sub_type)) => sub_type , _ => & field . ty , } ; let updater = quote_spanned ! { ty . span () => <# subcmd_type as clap :: FromArgMatches >:: update_from_arg_matches_mut (# field_name , # arg_matches) ?; } ; let updater = match * * ty { Ty :: Option => quote_spanned ! { kind . span () => if let Some (# field_name) = # field_name . as_mut () { # updater } else { *# field_name = Some (<# subcmd_type as clap :: FromArgMatches >:: from_arg_matches_mut (# arg_matches) ?) ; } } , _ => quote_spanned ! { kind . span () => # updater } , } ; quote_spanned ! { kind . span () => { # access # updater } } } Kind :: Flatten (ty) => { let inner_type = match (* * ty , sub_type (& field . ty)) { (Ty :: Option , Some (sub_type)) => sub_type , _ => & field . ty , } ; let updater = quote_spanned ! { ty . span () => <# inner_type as clap :: FromArgMatches >:: update_from_arg_matches_mut (# field_name , # arg_matches) ?; } ; let updater = match * * ty { Ty :: Option => quote_spanned ! { kind . span () => if let Some (# field_name) = # field_name . as_mut () { # updater } else { *# field_name = Some (<# inner_type as clap :: FromArgMatches >:: from_arg_matches_mut (# arg_matches) ?) ; } } , _ => quote_spanned ! { kind . span () => # updater } , } ; quote_spanned ! { kind . span () => { # access # updater } } } Kind :: Skip (_ , _) => quote ! () , Kind :: Arg (ty) | Kind :: FromGlobal (ty) => { gen_parsers (item , ty , field_name , field , Some (& access)) ? } } ; genned_fields . push (genned) ; } Ok (quote ! { # (# genned_fields) * }) }
    };
}

gen_updater!();