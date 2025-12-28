macro_rules! deps {
    () => {
        Http!();
        Service!();
    };
}

macro_rules! HeadersThenBody {
    () => {
        deps!();
        struct HeadersThenBody < H : Http , B : Unpin > { service : Service , headers : Option < H :: Headers > , body : B , }
    };
}

HeadersThenBody!()