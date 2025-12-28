macro_rules! deps {
    () => {
        SocketEvents!();
        Socket!();
    };
}

macro_rules! MultiData {
    () => {
        deps!();
        struct MultiData { socket : Box < dyn FnMut (Socket , SocketEvents , usize) + Send > , timer : Box < dyn FnMut (Option < Duration >) -> bool + Send > , }
    };
}

MultiData!()