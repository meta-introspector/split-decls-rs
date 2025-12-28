macro_rules! ErrorKind {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] enum ErrorKind { Empty , InvalidDigit , Overflow , Underflow , }
    };
}

ErrorKind!()