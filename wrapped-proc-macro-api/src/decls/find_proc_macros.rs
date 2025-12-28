macro_rules! deps {
    () => {
        ProcMacroServerProcess!();
        ServerError!();
        ProcMacroKind!();
    };
}

macro_rules! find_proc_macros {
    () => {
        deps!();
        # [doc = " Finds proc-macros in a given dynamic library."] pub (crate) fn find_proc_macros (srv : & ProcMacroServerProcess , dylib_path : & AbsPath ,) -> Result < Result < Vec < (String , ProcMacroKind) > , String > , ServerError > { let request = Request :: ListMacros { dylib_path : dylib_path . to_path_buf () . into () } ; let response = send_task (srv , request) ? ; match response { Response :: ListMacros (it) => Ok (it) , _ => Err (ServerError { message : "unexpected response" . to_owned () , io : None }) , } }
    };
}

find_proc_macros!()