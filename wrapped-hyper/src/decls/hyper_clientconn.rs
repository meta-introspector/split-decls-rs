macro_rules! deps {
    () => {
        Tx!();
    };
}

macro_rules! hyper_clientconn {
    () => {
        deps!();
        # [doc = " An HTTP client connection handle."] # [doc = ""] # [doc = " These are used to send one or more requests on a single connection."] # [doc = ""] # [doc = " It's possible to send multiple requests on a single connection, such"] # [doc = " as when HTTP/1 keep-alive or HTTP/2 is used."] # [doc = ""] # [doc = " To create a `hyper_clientconn`:"] # [doc = ""] # [doc = "   1. Create a `hyper_io` with `hyper_io_new`."] # [doc = "   2. Create a `hyper_clientconn_options` with `hyper_clientconn_options_new`."] # [doc = "   3. Call `hyper_clientconn_handshake` with the `hyper_io` and `hyper_clientconn_options`."] # [doc = "      This creates a `hyper_task`."] # [doc = "   5. Call `hyper_task_set_userdata` to assign an application-specific pointer to the task."] # [doc = "      This allows keeping track of multiple connections that may be handshaking"] # [doc = "      simultaneously."] # [doc = "   4. Add the `hyper_task` to an executor with `hyper_executor_push`."] # [doc = "   5. Poll that executor until it yields a task of type `HYPER_TASK_CLIENTCONN`."] # [doc = "   6. Extract the `hyper_clientconn` from the task with `hyper_task_value`."] # [doc = "      This will require a cast from `void *` to `hyper_clientconn *`."] # [doc = ""] # [doc = " This process results in a `hyper_clientconn` that permanently owns the"] # [doc = " `hyper_io`. Because the `hyper_io` in turn owns a TCP or TLS connection, that means"] # [doc = " the `hyper_clientconn` owns the connection for both the clientconn's lifetime"] # [doc = " and the connection's lifetime."] # [doc = ""] # [doc = " In other words, each connection (`hyper_io`) must have exactly one `hyper_clientconn`"] # [doc = " associated with it. That's because `hyper_clientconn_handshake` sends the"] # [doc = " [HTTP/2 Connection Preface] (for HTTP/2 connections). Since that preface can't"] # [doc = " be sent twice, handshake can't be called twice."] # [doc = ""] # [doc = " [HTTP/2 Connection Preface]: https://datatracker.ietf.org/doc/html/rfc9113#name-http-2-connection-preface"] # [doc = ""] # [doc = " Methods:"] # [doc = ""] # [doc = " - hyper_clientconn_handshake:  Creates an HTTP client handshake task."] # [doc = " - hyper_clientconn_send:       Creates a task to send a request on the client connection."] # [doc = " - hyper_clientconn_free:       Free a hyper_clientconn *."] pub struct hyper_clientconn { tx : Tx , }
    };
}

hyper_clientconn!();