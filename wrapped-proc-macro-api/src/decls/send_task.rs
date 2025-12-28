macro_rules! deps {
    () => {
        ServerError!();
        ProcMacroServerProcess!();
    };
}

macro_rules! send_task {
    () => {
        deps!();
        # [doc = " Sends a request to the proc-macro server and waits for a response."] fn send_task (srv : & ProcMacroServerProcess , req : Request) -> Result < Response , ServerError > { if let Some (server_error) = srv . exited () { return Err (server_error . clone ()) ; } srv . send_task (send_request , req) }
    };
}

send_task!();