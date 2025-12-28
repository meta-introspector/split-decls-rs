macro_rules! deps {
    () => {
        ServerError!();
        ProcMacroServerProcess!();
    };
}

macro_rules! version_check {
    () => {
        deps!();
        pub (crate) fn version_check (srv : & ProcMacroServerProcess) -> Result < u32 , ServerError > { let request = Request :: ApiVersionCheck { } ; let response = send_task (srv , request) ? ; match response { Response :: ApiVersionCheck (version) => Ok (version) , _ => Err (ServerError { message : "unexpected response" . to_owned () , io : None }) , } }
    };
}

version_check!()