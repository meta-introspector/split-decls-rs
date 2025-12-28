macro_rules! deps {
    () => {
        ArgValueCandidates!();
        ArgValueCompleter!();
        CompletionCandidate!();
    };
}

macro_rules! complete_arg_value {
    () => {
        deps!();
        fn complete_arg_value (value : Result < & str , & OsStr > , arg : & clap :: Arg , current_dir : Option < & std :: path :: Path > ,) -> Vec < CompletionCandidate > { let mut values = Vec :: new () ; debug ! ("complete_arg_value: arg={arg:?}, value={value:?}") ; let (prefix , value) = rsplit_delimiter (value , arg . get_value_delimiter ()) . unwrap_or ((None , value)) ; let value_os = match value { Ok (value) => OsStr :: new (value) , Err (value_os) => value_os , } ; if let Some (completer) = arg . get :: < ArgValueCompleter > () { values . extend (completer . complete (value_os)) ; } else if let Some (completer) = arg . get :: < ArgValueCandidates > () { values . extend (complete_custom_arg_value (value_os , completer)) ; } else if let Some (possible_values) = possible_values (arg) { if let Ok (value) = value { values . extend (possible_values . into_iter () . filter_map (| p | { let name = p . get_name () ; name . starts_with (value) . then (| | { CompletionCandidate :: new (OsString :: from (name)) . help (p . get_help () . cloned ()) . hide (p . is_hide_set ()) }) })) ; } } else { match arg . get_value_hint () { clap :: ValueHint :: Unknown | clap :: ValueHint :: Other => { } clap :: ValueHint :: AnyPath => { values . extend (complete_path (value_os , current_dir , & | _ | true)) ; } clap :: ValueHint :: FilePath => { values . extend (complete_path (value_os , current_dir , & | p | p . is_file ())) ; } clap :: ValueHint :: DirPath => { values . extend (complete_path (value_os , current_dir , & | p | p . is_dir ())) ; } clap :: ValueHint :: ExecutablePath => { use is_executable :: IsExecutable ; values . extend (complete_path (value_os , current_dir , & | p | p . is_executable ())) ; } clap :: ValueHint :: CommandName | clap :: ValueHint :: CommandString | clap :: ValueHint :: CommandWithArguments | clap :: ValueHint :: Username | clap :: ValueHint :: Hostname | clap :: ValueHint :: Url | clap :: ValueHint :: EmailAddress => { } _ => { values . extend (complete_path (value_os , current_dir , & | _ | true)) ; } } values . sort () ; } if let Some (prefix) = prefix { values = values . into_iter () . map (| comp | comp . add_prefix (prefix)) . collect () ; } values = values . into_iter () . map (| comp | { if comp . get_tag () . is_some () { comp } else { comp . tag (Some (arg . to_string () . into ())) } }) . collect () ; values }
    };
}

complete_arg_value!()