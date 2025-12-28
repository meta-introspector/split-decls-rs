macro_rules! search_for_section {
    () => {
        pub (super) fn search_for_section < 'a > (path : & Path , bytes : & 'a [u8] , section : & str ,) -> Result < & 'a [u8] , String > { let Ok (file) = object :: File :: parse (bytes) else { return Ok (bytes) ; } ; file . section_by_name (section) . ok_or_else (| | format ! ("no `{}` section in '{}'" , section , path . display ())) ? . data () . map_err (| e | format ! ("failed to read {} section in '{}': {}" , section , path . display () , e)) }
    };
}

search_for_section!();