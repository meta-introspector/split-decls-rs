macro_rules! deps {
    () => {
        TransportFactory!();
    };
}

macro_rules! TransportData {
    () => {
        deps!();
        # [doc = " Boxed data payload used for registering new transports."] # [doc = ""] # [doc = " Currently only contains a field which knows how to create transports."] struct TransportData { factory : Box < TransportFactory > , }
    };
}

TransportData!()