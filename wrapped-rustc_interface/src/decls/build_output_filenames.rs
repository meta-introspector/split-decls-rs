macro_rules! deps {
    () => {
        IgnoringExtraFilename!();
        MultipleOutputTypesToStdout!();
        IgnoringOutDir!();
        MultipleOutputTypesAdaption!();
    };
}

macro_rules! build_output_filenames {
    () => {
        deps!();
        pub fn build_output_filenames (attrs : & [ast :: Attribute] , sess : & Session) -> OutputFilenames { if multiple_output_types_to_stdout (& sess . opts . output_types , sess . io . output_file == Some (OutFileName :: Stdout) ,) { sess . dcx () . emit_fatal (errors :: MultipleOutputTypesToStdout) ; } let crate_name = sess . opts . crate_name . clone () . or_else (| | { parse_crate_name (sess , attrs , ShouldEmit :: Nothing) . map (| i | i . 0 . to_string ()) }) ; match sess . io . output_file { None => { let dirpath = sess . io . output_dir . clone () . unwrap_or_default () ; let stem = crate_name . clone () . unwrap_or_else (| | sess . io . input . filestem () . to_owned ()) ; OutputFilenames :: new (dirpath , crate_name . unwrap_or_else (| | stem . replace ('-' , "_")) , stem , None , sess . io . temps_dir . clone () , sess . opts . cg . extra_filename . clone () , sess . opts . output_types . clone () ,) } Some (ref out_file) => { let unnamed_output_types = sess . opts . output_types . values () . filter (| a | a . is_none ()) . count () ; let ofile = if unnamed_output_types > 1 { sess . dcx () . emit_warn (errors :: MultipleOutputTypesAdaption) ; None } else { if ! sess . opts . cg . extra_filename . is_empty () { sess . dcx () . emit_warn (errors :: IgnoringExtraFilename) ; } Some (out_file . clone ()) } ; if sess . io . output_dir != None { sess . dcx () . emit_warn (errors :: IgnoringOutDir) ; } let out_filestem = out_file . filestem () . unwrap_or_default () . to_str () . unwrap () . to_string () ; OutputFilenames :: new (out_file . parent () . unwrap_or_else (| | Path :: new ("")) . to_path_buf () , crate_name . unwrap_or_else (| | out_filestem . replace ('-' , "_")) , out_filestem , ofile , sess . io . temps_dir . clone () , sess . opts . cg . extra_filename . clone () , sess . opts . output_types . clone () ,) } } }
    };
}

build_output_filenames!()