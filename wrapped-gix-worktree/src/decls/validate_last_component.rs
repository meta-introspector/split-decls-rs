macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! validate_last_component {
    () => {
        deps!();
        # [cfg (feature = "attributes")] fn validate_last_component (stack : & gix_fs :: Stack , mode : Option < gix_index :: entry :: Mode > , opts : gix_validate :: path :: component :: Options ,) -> std :: io :: Result < () > { let Some (last_component) = stack . current_relative () . components () . next_back () else { return Ok (()) ; } ; let last_component = gix_path :: try_into_bstr (std :: borrow :: Cow :: Borrowed (last_component . as_os_str () . as_ref ())) . map_err (| _err | { std :: io :: Error :: other (format ! ("Path component {last_component:?} of path \"{}\" contained invalid UTF-8 and could not be validated" , stack . current_relative () . display ())) }) ? ; if let Err (err) = gix_validate :: path :: component (last_component . as_ref () , mode . and_then (| m | { (m == gix_index :: entry :: Mode :: SYMLINK) . then_some (gix_validate :: path :: component :: Mode :: Symlink) }) , opts ,) { return Err (std :: io :: Error :: other (err)) ; } Ok (()) }
    };
}

validate_last_component!();