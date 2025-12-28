macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl PartialEq for IUnknown { fn eq (& self , other : & Self) -> bool { core :: ptr :: eq (self . as_raw () , other . as_raw ()) || self . cast :: < Self > () . unwrap () . 0 == other . cast :: < Self > () . unwrap () . 0 } }
    };
}

impl_204!();