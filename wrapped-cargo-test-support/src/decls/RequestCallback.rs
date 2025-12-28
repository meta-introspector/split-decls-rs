macro_rules! deps {
    () => {
        Response!();
        Request!();
        HttpServer!();
    };
}

macro_rules! RequestCallback {
    () => {
        deps!();
        type RequestCallback = Box < dyn Send + Fn (& Request , & HttpServer) -> Response > ;
    };
}

RequestCallback!();