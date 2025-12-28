macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! handshake {
    () => {
        deps!();
        # [doc = ""] pub mod handshake { # [doc = " The error returned by [Server::handshake()][super::Server::handshake()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to read or write to the client")] Io (# [from] std :: io :: Error) , # [error ("{msg} '{actual}'")] Protocol { msg : String , actual : String } , # [error ("Could not select supported version from the one sent by the client: {}" , actual . iter () . map (ToString :: to_string) . collect ::< Vec < _ >> () . join (", "))] VersionMismatch { actual : Vec < usize > } , } }
    };
}

handshake!()