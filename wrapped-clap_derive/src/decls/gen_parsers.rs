macro_rules! deps {
    () => {
        Name!();
        Ty!();
        Item!();
        Sp!();
    };
}

macro_rules! gen_parsers {
    () => {
        deps!();
        fn gen_parsers (item : & Item , ty : & Sp < Ty > , field_name : & Ident , field : & Field , update : Option < & TokenStream > ,) -> Result < TokenStream , syn :: Error > { let span = ty . span () ; let convert_type = inner_type (& field . ty) ; let id = item . id () ; let get_one = quote_spanned ! (span => remove_one ::<# convert_type >) ; let get_many = quote_spanned ! (span => remove_many ::<# convert_type >) ; let get_occurrences = quote_spanned ! (span => remove_occurrences ::<# convert_type >) ; let arg_matches = format_ident ! ("__clap_arg_matches") ; let field_value = match * * ty { Ty :: Unit => { quote_spanned ! { ty . span () => () } } Ty :: Option => { quote_spanned ! { ty . span () => # arg_matches .# get_one (# id) } } Ty :: OptionOption => quote_spanned ! { ty . span () => if # arg_matches . contains_id (# id) { Some (# arg_matches .# get_one (# id)) } else { None } } , Ty :: OptionVec => quote_spanned ! { ty . span () => if # arg_matches . contains_id (# id) { Some (# arg_matches .# get_many (# id) . map (| v | v . collect ::< Vec < _ >> ()) . unwrap_or_else (Vec :: new)) } else { None } } , Ty :: Vec => { quote_spanned ! { ty . span () => # arg_matches .# get_many (# id) . map (| v | v . collect ::< Vec < _ >> ()) . unwrap_or_else (Vec :: new) } } Ty :: VecVec => quote_spanned ! { ty . span () => # arg_matches .# get_occurrences (# id) . map (| g | g . map (:: std :: iter :: Iterator :: collect) . collect ::< Vec < Vec < _ >>> ()) . unwrap_or_else (Vec :: new) } , Ty :: OptionVecVec => quote_spanned ! { ty . span () => # arg_matches .# get_occurrences (# id) . map (| g | g . map (:: std :: iter :: Iterator :: collect) . collect ::< Vec < Vec < _ >>> ()) } , Ty :: Other => { match id { Name :: Assigned (_) => { quote_spanned ! { ty . span () => # arg_matches .# get_one (# id) . ok_or_else (|| clap :: Error :: raw (clap :: error :: ErrorKind :: MissingRequiredArgument , format ! ("the following required argument was not provided: {}" , # id))) ? } } Name :: Derived (_) => { quote_spanned ! { ty . span () => # arg_matches .# get_one (# id) . ok_or_else (|| clap :: Error :: raw (clap :: error :: ErrorKind :: MissingRequiredArgument , concat ! ("the following required argument was not provided: " , # id))) ? } } } } } ; let genned = if let Some (access) = update { quote_spanned ! { field . span () => if # arg_matches . contains_id (# id) { # access *# field_name = # field_value } } } else { quote_spanned ! (field . span () => # field_name : # field_value) } ; Ok (genned) }
    };
}

gen_parsers!()