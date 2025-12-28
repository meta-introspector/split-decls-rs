macro_rules! deps {
    () => {
        JoinLinesConfig!();
    };
}

macro_rules! remove_newlines {
    () => {
        deps!();
        fn remove_newlines (config : & JoinLinesConfig , edit : & mut TextEditBuilder , token : & SyntaxToken , range : TextRange ,) { let intersection = match range . intersect (token . text_range ()) { Some (range) => range , None => return , } ; let range = intersection - token . text_range () . start () ; let text = token . text () ; for (pos , _) in text [range] . bytes () . enumerate () . filter (| & (_ , b) | b == b'\n') { let pos : TextSize = (pos as u32) . into () ; let offset = token . text_range () . start () + range . start () + pos ; if ! edit . invalidates_offset (offset) { remove_newline (config , edit , token , offset) ; } } }
    };
}

remove_newlines!();