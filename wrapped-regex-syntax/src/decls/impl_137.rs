macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
        Formatter!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'e > From < & 'e hir :: Error > for Formatter < 'e , hir :: ErrorKind > { fn from (err : & 'e hir :: Error) -> Self { Formatter { pattern : err . pattern () , err : err . kind () , span : err . span () , aux_span : None , } } }
    };
}

impl_137!();