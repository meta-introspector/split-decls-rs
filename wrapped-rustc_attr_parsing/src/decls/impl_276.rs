macro_rules! deps {
    () => {
        AcceptContext!();
        SharedContext!();
        Stage!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < 'f , 'sess , S : Stage > Deref for AcceptContext < 'f , 'sess , S > { type Target = SharedContext < 'f , 'sess , S > ; fn deref (& self) -> & Self :: Target { & self . shared } }
    };
}

impl_276!()