macro_rules! deps {
    () => {
        Outcome!();
        Entry!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < & Entry > for Outcome { fn from (e : & Entry) -> Self { Outcome { status : e . status , property : e . property , disk_kind : e . disk_kind , index_kind : e . index_kind , pathspec_match : e . pathspec_match , } } }
    };
}

impl_28!()