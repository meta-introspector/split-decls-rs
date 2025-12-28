macro_rules! deps {
    () => {
        Result!();
        Node!();
        SpanError!();
        Context!();
    };
}

macro_rules! get_format_args {
    () => {
        deps!();
        # [doc = " Common code shared between the public macros, terminfo implementation."] pub fn get_format_args (input : TokenStream) -> Result < TokenStream2 , SpanError > { let (format_string_token , args) = get_args_and_format_string (input) ? ; let format_string = format_string_token . value () ; let format_nodes = parse_format_string (& format_string , & format_string_token) ? ; let mut final_format_string = String :: new () ; let mut color_context = Context :: new () ; let mut current_color_idx = 0 ; let mut color_format_args : Vec < TokenStream2 > = vec ! [] ; for node in format_nodes { match node { Node :: Text (s) | Node :: Placeholder (s) => { final_format_string . push_str (s) ; } Node :: ColorTagGroup (tag_group) => { let constants = color_context . terminfo_apply_tags (tag_group) ? . iter () . map (| s | constant_to_token_stream (s)) . collect :: < Vec < _ > > () ; for constant in constants { let varname = format ! ("__color_print__color_{}" , current_color_idx) ; final_format_string . push_str (& format ! ("{{{}}}" , varname)) ; current_color_idx += 1 ; let varname_ident = util :: ident (& varname) ; let token_stream = quote ! { # varname_ident = # constant } . into () ; color_format_args . push (token_stream) ; } } } } let format_string_span = format_string_token . span () ; let final_format_string = LitStr :: new (& final_format_string , format_string_span) . to_token_stream () ; let final_args = std :: iter :: once (final_format_string) . chain (args . iter () . map (| arg | arg . to_token_stream ()) . skip (1)) . chain (color_format_args . into_iter ()) ; Ok ((quote ! { # (# final_args) ,* }) . into ()) }
    };
}

get_format_args!()