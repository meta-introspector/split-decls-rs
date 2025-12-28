macro_rules! report_format_mismatch {
    () => {
        fn report_format_mismatch (report_incremental_info : bool , file : & Path , message : & str) { debug ! ("read_file: {}" , message) ; if report_incremental_info { eprintln ! ("[incremental] ignoring cache artifact `{}`: {}" , file . file_name () . unwrap () . to_string_lossy () , message) ; } }
    };
}

report_format_mismatch!();