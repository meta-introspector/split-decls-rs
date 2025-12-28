macro_rules! deps {
    () => {
        AcceptContext!();
        Stage!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < 'f , 'sess , S : Stage > DerefMut for AcceptContext < 'f , 'sess , S > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . shared } }
    };
}

impl_277!();