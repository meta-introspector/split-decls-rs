macro_rules! deps {
    () => {
        AutoFinisher!();
        AutoFinish!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T : AutoFinish > core :: ops :: DerefMut for AutoFinisher < T > { fn deref_mut (& mut self) -> & mut Self :: Target { self . 0 . as_mut () . unwrap () } }
    };
}

impl_84!()