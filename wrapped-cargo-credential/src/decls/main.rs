macro_rules! deps {
    () => {
        Credential!();
    };
}

macro_rules! main {
    () => {
        deps!();
        # [doc = " Runs the credential interaction"] pub fn main (credential : impl Credential) { let result = doit (credential) . map_err (| e | Error :: Other (e)) ; if result . is_err () { serde_json :: to_writer (std :: io :: stdout () , & result) . expect ("failed to serialize credential provider error") ; println ! () ; } }
    };
}

main!()