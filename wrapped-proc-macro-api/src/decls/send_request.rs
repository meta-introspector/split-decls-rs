macro_rules! deps {
    () => {
        ServerError!();
    };
}

macro_rules! send_request {
    () => {
        deps!();
        # [doc = " Sends a request to the server and reads the response."] fn send_request (mut writer : & mut dyn Write , mut reader : & mut dyn BufRead , req : Request , buf : & mut String ,) -> Result < Option < Response > , ServerError > { req . write (write_json , & mut writer) . map_err (| err | ServerError { message : "failed to write request" . into () , io : Some (Arc :: new (err)) , }) ? ; let res = Response :: read (read_json , & mut reader , buf) . map_err (| err | ServerError { message : "failed to read response" . into () , io : Some (Arc :: new (err)) , }) ? ; Ok (res) }
    };
}

send_request!()