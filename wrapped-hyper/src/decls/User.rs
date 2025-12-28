macro_rules! deps {
    () => {
        Service!();
        Error!();
    };
}

macro_rules! User {
    () => {
        deps!();
        # [derive (Debug)] pub (super) enum User { # [doc = " Error calling user's Body::poll_data()."] # [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2")))] Body , # [doc = " The user aborted writing of the outgoing body."] # [cfg (any (all (feature = "http1" , any (feature = "client" , feature = "server")) , feature = "ffi"))] BodyWriteAborted , # [doc = " User tried to send a connect request with a nonzero body"] # [cfg (all (feature = "client" , feature = "http2"))] InvalidConnectWithBody , # [doc = " Error from future of user's Service."] # [cfg (any (all (any (feature = "client" , feature = "server") , feature = "http1") , all (feature = "server" , feature = "http2")))] Service , # [doc = " User tried to send a certain header in an unexpected context."] # [doc = ""] # [doc = " For example, sending both `content-length` and `transfer-encoding`."] # [cfg (any (feature = "http1" , feature = "http2"))] # [cfg (feature = "server")] UnexpectedHeader , # [doc = " User tried to respond with a 1xx (not 101) response code."] # [cfg (feature = "http1")] # [cfg (feature = "server")] UnsupportedStatusCode , # [doc = " User tried polling for an upgrade that doesn't exist."] NoUpgrade , # [doc = " User polled for an upgrade, but low-level API is not using upgrades."] # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] ManualUpgrade , # [doc = " The dispatch task is gone."] # [cfg (all (feature = "client" , any (feature = "http1" , feature = "http2")))] DispatchGone , # [doc = " User aborted in an FFI callback."] # [cfg (feature = "ffi")] AbortedByCallback , }
    };
}

User!()