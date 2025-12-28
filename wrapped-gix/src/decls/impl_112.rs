macro_rules! deps {
    () => {
        EntryRef!();
        Status!();
        Item!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Item { fn new (entry : gix_dir :: EntryRef < '_ > , collapsed_directory_status : Option < gix_dir :: entry :: Status >) -> Self { Item { entry : entry . to_owned () , collapsed_directory_status , } } }
    };
}

impl_112!()