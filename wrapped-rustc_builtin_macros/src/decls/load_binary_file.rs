macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! load_binary_file {
    () => {
        deps!();
        fn load_binary_file (cx : & ExtCtxt < '_ > , original_path : & Path , macro_span : Span , path_span : Span ,) -> Result < (Arc < [u8] > , Span) , Box < dyn MacResult > > { let resolved_path = match resolve_path (& cx . sess , original_path , macro_span) { Ok (path) => path , Err (err) => { let guar = err . emit () ; return Err (DummyResult :: any (macro_span , guar)) ; } } ; match cx . source_map () . load_binary_file (& resolved_path) { Ok (data) => Ok (data) , Err (io_err) => { let mut err = cx . dcx () . struct_span_err (macro_span , format ! ("couldn't read `{}`: {io_err}" , resolved_path . display ()) ,) ; if original_path . is_relative () { let source_map = cx . sess . source_map () ; let new_path = source_map . span_to_filename (macro_span . source_callsite ()) . into_local_path () . and_then (| src | find_path_suggestion (source_map , src . parent () ? , original_path)) . and_then (| path | path . into_os_string () . into_string () . ok ()) ; if let Some (new_path) = new_path { err . span_suggestion_verbose (path_span , "there is a file with the same name in a different directory" , format ! ("\"{}\"" , new_path . replace ('\\' , "/") . escape_debug ()) , rustc_lint_defs :: Applicability :: MachineApplicable ,) ; } } let guar = err . emit () ; Err (DummyResult :: any (macro_span , guar)) } } }
    };
}

load_binary_file!();