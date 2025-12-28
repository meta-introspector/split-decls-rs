macro_rules! deps {
    () => {
        CrateNameInvalid!();
        CrateNameDoesNotMatch!();
    };
}

macro_rules! get_crate_name {
    () => {
        deps!();
        # [doc = " Compute and validate the crate name."] pub fn get_crate_name (sess : & Session , krate_attrs : & [ast :: Attribute]) -> Symbol { let attr_crate_name = parse_crate_name (sess , krate_attrs , ShouldEmit :: EarlyFatal { also_emit_lints : true }) ; let validate = | name , span | { rustc_session :: output :: validate_crate_name (sess , name , span) ; name } ; if let Some (crate_name) = & sess . opts . crate_name { let crate_name = Symbol :: intern (crate_name) ; if let Some ((attr_crate_name , span)) = attr_crate_name && attr_crate_name != crate_name { sess . dcx () . emit_err (errors :: CrateNameDoesNotMatch { span , crate_name , attr_crate_name , }) ; } return validate (crate_name , None) ; } if let Some ((crate_name , span)) = attr_crate_name { return validate (crate_name , Some (span)) ; } if let Input :: File (ref path) = sess . io . input && let Some (file_stem) = path . file_stem () . and_then (| s | s . to_str ()) { if file_stem . starts_with ('-') { sess . dcx () . emit_err (errors :: CrateNameInvalid { crate_name : file_stem }) ; } else { return validate (Symbol :: intern (& file_stem . replace ('-' , "_")) , None) ; } } sym :: rust_out }
    };
}

get_crate_name!();