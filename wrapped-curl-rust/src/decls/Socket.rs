macro_rules! Socket {
    () => {
        # [doc = " Raw underlying socket type that the multi handles use"] pub type Socket = curl_sys :: curl_socket_t ;
    };
}

Socket!();