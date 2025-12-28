macro_rules! deps {
    () => {
        Formatter!();
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < 'e > From < & 'e ast :: Error > for Formatter < 'e , ast :: ErrorKind > { fn from (err : & 'e ast :: Error) -> Self { Formatter { pattern : err . pattern () , err : err . kind () , span : err . span () , aux_span : err . auxiliary_span () , } } }
    };
}

impl_136!()