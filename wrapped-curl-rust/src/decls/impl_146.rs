macro_rules! deps {
    () => {
        SocketEvents!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl SocketEvents { # [doc = " Wait for incoming data. For the socket to become readable."] pub fn input (& self) -> bool { self . bits & curl_sys :: CURL_POLL_IN == curl_sys :: CURL_POLL_IN } # [doc = " Wait for outgoing data. For the socket to become writable."] pub fn output (& self) -> bool { self . bits & curl_sys :: CURL_POLL_OUT == curl_sys :: CURL_POLL_OUT } # [doc = " Wait for incoming and outgoing data. For the socket to become readable"] # [doc = " or writable."] pub fn input_and_output (& self) -> bool { self . bits & curl_sys :: CURL_POLL_INOUT == curl_sys :: CURL_POLL_INOUT } # [doc = " The specified socket/file descriptor is no longer used by libcurl."] pub fn remove (& self) -> bool { self . bits & curl_sys :: CURL_POLL_REMOVE == curl_sys :: CURL_POLL_REMOVE } }
    };
}

impl_146!();