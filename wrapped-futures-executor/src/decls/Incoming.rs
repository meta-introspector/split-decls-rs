macro_rules! Incoming {
    () => {
        type Incoming = RefCell < Vec < LocalFutureObj < 'static , () > > > ;
    };
}

Incoming!();