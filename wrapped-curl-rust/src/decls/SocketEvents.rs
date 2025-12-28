macro_rules! SocketEvents {
    () => {
        # [doc = " Notification of events that are requested on a socket."] # [doc = ""] # [doc = " This type is yielded to the `socket_function` callback to indicate what"] # [doc = " events are requested on a socket."] pub struct SocketEvents { bits : c_int , }
    };
}

SocketEvents!()