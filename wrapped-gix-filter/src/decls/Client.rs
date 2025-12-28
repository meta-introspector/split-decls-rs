macro_rules! deps {
    () => {
        Capabilities!();
    };
}

macro_rules! Client {
    () => {
        deps!();
        # [doc = " A handle to a client that allows communicating to a long-running process."] pub struct Client { # [doc = " The child process we are communicating with."] child : std :: process :: Child , # [doc = " The names of the obtained capabilities after the handshake."] capabilities : Capabilities , # [doc = " The negotiated version of the protocol."] version : usize , # [doc = " A way to send packet-line encoded information to the process."] input : Writer < std :: process :: ChildStdin > , # [doc = " A way to read information sent to us by the process."] out : StreamingPeekableIter < std :: process :: ChildStdout > , }
    };
}

Client!()