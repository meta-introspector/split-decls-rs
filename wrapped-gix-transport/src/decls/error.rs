macro_rules! deps {
    () => {
        Http!();
        Error!();
        Capabilities!();
    };
}

macro_rules! error {
    () => {
        deps!();
        mod error { use std :: ffi :: OsString ; use bstr :: BString ; # [cfg (feature = "http-client")] use crate :: client :: blocking_io :: http ; # [cfg (feature = "blocking-client")] use crate :: client :: blocking_io :: ssh ; use crate :: client :: capabilities ; # [cfg (feature = "http-client")] type HttpError = http :: Error ; # [cfg (feature = "blocking-client")] type SshInvocationError = ssh :: invocation :: Error ; # [cfg (not (feature = "http-client"))] type HttpError = std :: convert :: Infallible ; # [cfg (not (feature = "blocking-client"))] type SshInvocationError = std :: convert :: Infallible ; # [doc = " The error used in most methods of the [`client`][crate::client] module"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("A request was performed without performing the handshake first")] MissingHandshake , # [error ("An IO error occurred when talking to the server")] Io (# [from] std :: io :: Error) , # [error ("Capabilities could not be parsed")] Capabilities { # [from] err : capabilities :: Error , } , # [error ("A packet line could not be decoded")] LineDecode { # [from] err : gix_packetline :: decode :: Error , } , # [error ("A {0} line was expected, but there was none")] ExpectedLine (& 'static str) , # [error ("Expected a data line, but got a delimiter")] ExpectedDataLine , # [error ("The transport layer does not support authentication")] AuthenticationUnsupported , # [error ("The transport layer refuses to use a given identity: {0}")] AuthenticationRefused (& 'static str) , # [error ("The protocol version indicated by {:?} is unsupported" , { 0 })] UnsupportedProtocolVersion (BString) , # [error ("Failed to invoke program {command:?}")] InvokeProgram { source : std :: io :: Error , command : OsString } , # [error (transparent)] Http (# [from] HttpError) , # [error (transparent)] SshInvocation (SshInvocationError) , # [error ("The repository path '{path}' could be mistaken for a command-line argument")] AmbiguousPath { path : BString } , } impl crate :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Io (err) => err . is_spurious () , Error :: Http (err) => err . is_spurious () , _ => false , } } } }
    };
}

error!();