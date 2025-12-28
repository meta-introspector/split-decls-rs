macro_rules! close_tempfile_and_log_error {
    () => {
        fn close_tempfile_and_log_error (file : NamedTempFile) { file . close () . unwrap_or_else (| e | { tracing :: warn ! ("failed to close temporary file: {e}") ; }) ; }
    };
}

close_tempfile_and_log_error!()