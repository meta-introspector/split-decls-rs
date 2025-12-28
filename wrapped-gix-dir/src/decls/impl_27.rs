macro_rules! deps {
    () => {
        Kind!();
        Status!();
        Outcome!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Outcome { fn with_status (mut self , status : entry :: Status) -> Self { self . status = status ; self } fn with_kind (mut self , disk_kind : Option < entry :: Kind > , index_kind : Option < entry :: Kind >) -> Self { self . disk_kind = disk_kind ; self . index_kind = index_kind ; self } }
    };
}

impl_27!()