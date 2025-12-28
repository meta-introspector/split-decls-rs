macro_rules! deps {
    () => {
        ProcessError!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl ProcessError { # [doc = " Creates a new [`ProcessError`]."] # [doc = ""] # [doc = " * `status` can be `None` if the process did not launch."] # [doc = " * `output` can be `None` if the process did not launch, or output was not captured."] pub fn new (msg : & str , status : Option < ExitStatus > , output : Option < & Output >) -> ProcessError { let exit = match status { Some (s) => exit_status_to_string (s) , None => "never executed" . to_string () , } ; Self :: new_raw (msg , status . and_then (| s | s . code ()) , & exit , output . map (| s | s . stdout . as_slice ()) , output . map (| s | s . stderr . as_slice ()) ,) } # [doc = " Creates a new [`ProcessError`] with the raw output data."] # [doc = ""] # [doc = " * `code` can be `None` for situations like being killed by a signal on unix."] pub fn new_raw (msg : & str , code : Option < i32 > , status : & str , stdout : Option < & [u8] > , stderr : Option < & [u8] > ,) -> ProcessError { let mut desc = format ! ("{} ({})" , msg , status) ; if let Some (out) = stdout { match str :: from_utf8 (out) { Ok (s) if ! s . trim () . is_empty () => { desc . push_str ("\n--- stdout\n") ; desc . push_str (s) ; } Ok (..) | Err (..) => { } } } if let Some (out) = stderr { match str :: from_utf8 (out) { Ok (s) if ! s . trim () . is_empty () => { desc . push_str ("\n--- stderr\n") ; desc . push_str (s) ; } Ok (..) | Err (..) => { } } } ProcessError { desc , code , stdout : stdout . map (| s | s . to_vec ()) , stderr : stderr . map (| s | s . to_vec ()) , } } # [doc = " Creates a [`ProcessError`] with \"could not execute process {cmd}\"."] # [doc = ""] # [doc = " * `cmd` is usually but not limited to [`std::process::Command`]."] pub fn could_not_execute (cmd : impl fmt :: Display) -> ProcessError { ProcessError :: new (& format ! ("could not execute process {cmd}") , None , None) } }
    };
}

impl_65!()