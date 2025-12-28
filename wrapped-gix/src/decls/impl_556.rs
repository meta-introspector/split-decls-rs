macro_rules! deps {
    () => {
        CommitAutoRollback!();
    };
}

macro_rules! impl_556 {
    () => {
        deps!();
        impl Debug for CommitAutoRollback < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_str (& self . repo . as_ref () . expect ("still present") . config . resolved . to_string ()) } }
    };
}

impl_556!()