macro_rules! deps {
    () => {
        ExpandedTest!();
        Result!();
        Error!();
        Name!();
    };
}

macro_rules! expand_globs {
    () => {
        deps!();
        fn expand_globs (path : impl AsRef < Path >) -> Vec < ExpandedTest > { fn glob (pattern : & str) -> Result < Vec < PathBuf > > { let mut paths = glob :: glob (pattern) ? . map (| entry | entry . map_err (Error :: from)) . collect :: < Result < Vec < PathBuf > > > () ? ; paths . sort () ; Ok (paths) } fn bin_name (i : usize) -> Name { Name (format ! ("macrotest{:03}" , i)) } let mut vec = Vec :: new () ; let name = path . as_ref () . file_stem () . expect ("no file stem") . to_string_lossy () . to_string () ; let mut expanded = ExpandedTest { name : Name (name) , test : path . as_ref () . to_path_buf () , error : None , } ; if let Some (utf8) = path . as_ref () . to_str () { if utf8 . contains ('*') { match glob (utf8) { Ok (paths) => { for path in paths { vec . push (ExpandedTest { name : bin_name (vec . len ()) , test : path , error : None , }) ; } } Err (error) => expanded . error = Some (error) , } } else { vec . push (expanded) ; } } vec }
    };
}

expand_globs!()