macro_rules! deps {
    () => {
        AutoFinisher!();
        AutoFinish!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < T : AutoFinish > core :: ops :: Deref for AutoFinisher < T > { type Target = T ; fn deref (& self) -> & Self :: Target { self . 0 . as_ref () . unwrap () } }
    };
}

impl_342!();