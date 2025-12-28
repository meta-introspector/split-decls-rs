macro_rules! deps {
    () => {
        WeakExec!();
    };
}

macro_rules! hyper_clientconn_options {
    () => {
        deps!();
        # [doc = " An options builder to configure an HTTP client connection."] # [doc = ""] # [doc = " Methods:"] # [doc = ""] # [doc = " - hyper_clientconn_options_new:     Creates a new set of HTTP clientconn options to be used in a handshake."] # [doc = " - hyper_clientconn_options_exec:    Set the client background task executor."] # [doc = " - hyper_clientconn_options_http2:   Set whether to use HTTP2."] # [doc = " - hyper_clientconn_options_set_preserve_header_case:  Set whether header case is preserved."] # [doc = " - hyper_clientconn_options_set_preserve_header_order: Set whether header order is preserved."] # [doc = " - hyper_clientconn_options_http1_allow_multiline_headers: Set whether HTTP/1 connections accept obsolete line folding for header values."] # [doc = " - hyper_clientconn_options_free:    Free a set of HTTP clientconn options."] pub struct hyper_clientconn_options { http1_allow_obsolete_multiline_headers_in_responses : bool , http1_preserve_header_case : bool , http1_preserve_header_order : bool , http2 : bool , # [doc = " Use a `Weak` to prevent cycles."] exec : WeakExec , }
    };
}

hyper_clientconn_options!();