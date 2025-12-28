macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! curl_is_spurious {
    () => {
        deps!();
        pub (crate) fn curl_is_spurious (err : & curl :: Error) -> bool { err . is_couldnt_connect () || err . is_couldnt_resolve_proxy () || err . is_couldnt_resolve_host () || err . is_operation_timedout () || err . is_recv_error () || err . is_send_error () || err . is_http2_error () || err . is_http2_stream_error () || err . is_ssl_connect_error () || err . is_partial_file () }
    };
}

curl_is_spurious!();