macro_rules! deps {
    () => {
        ArArchiveBuilder!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a > ArArchiveBuilder < 'a > { pub fn new (sess : & 'a Session , object_reader : & 'static ObjectReader) -> ArArchiveBuilder < 'a > { ArArchiveBuilder { sess , object_reader , src_archives : vec ! [] , entries : vec ! [] } } }
    };
}

impl_31!();