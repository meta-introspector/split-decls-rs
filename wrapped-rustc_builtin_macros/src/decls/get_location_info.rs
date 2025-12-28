macro_rules! get_location_info {
    () => {
        fn get_location_info (cx : & ExtCtxt < '_ > , fn_ : & ast :: Fn) -> (Symbol , usize , usize , usize , usize) { let span = fn_ . ident . span ; let (source_file , lo_line , lo_col , hi_line , hi_col) = cx . sess . source_map () . span_to_location_info (span) ; let file_name = match source_file { Some (sf) => sf . name . display (FileNameDisplayPreference :: Remapped) . to_string () , None => "no-location" . to_string () , } ; (Symbol :: intern (& file_name) , lo_line , lo_col , hi_line , hi_col) }
    };
}

get_location_info!();