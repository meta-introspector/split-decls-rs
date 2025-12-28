macro_rules! deps {
    () => {
        CredentialHello!();
        Error!();
        Credential!();
    };
}

macro_rules! doit {
    () => {
        deps!();
        fn doit (credential : impl Credential ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { let hello = CredentialHello { v : vec ! [PROTOCOL_VERSION_1] , } ; serde_json :: to_writer (std :: io :: stdout () , & hello) ? ; println ! () ; loop { let mut buffer = String :: new () ; let len = std :: io :: stdin () . read_line (& mut buffer) ? ; if len == 0 { return Ok (()) ; } let request = deserialize_request (& buffer) ? ; let response = stdin_stdout_to_console (| | { credential . perform (& request . registry , & request . action , & request . args) }) ? ; serde_json :: to_writer (std :: io :: stdout () , & response) ? ; println ! () ; } }
    };
}

doit!();