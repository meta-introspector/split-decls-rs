macro_rules! expand_args {
    () => {
        # [track_caller] fn expand_args < I , S > (args : I) -> Vec < String > where I : IntoIterator < Item = S > , S : AsRef < str > , { # [track_caller] fn from_string (result : & mut Vec < String > , value : & str) { expand_args (result , value . split_whitespace () . map (| arg | arg . to_string ())) } # [track_caller] fn expand_args < I , S > (result : & mut Vec < String > , args : I) where I : IntoIterator < Item = S > , S : AsRef < str > , { let mut expand = false ; for arg in args . into_iter () . map (| arg | arg . as_ref () . to_string ()) { if arg . starts_with ('-') { expand = false ; } if expand { for args in io :: read_file_lines (& arg) { if ! args . starts_with ("//") { from_string (result , & args) ; } } } else if arg == "--etc" { expand = true ; } else { result . push (arg) ; } } } let mut result = vec ! [] ; expand_args (& mut result , args) ; result }
    };
}

expand_args!();