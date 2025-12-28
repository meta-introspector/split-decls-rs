macro_rules! deps {
    () => {
        Stage!();
        SharedContext!();
        FinalizeContext!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < 'p , 'sess : 'p , S : Stage > Deref for FinalizeContext < 'p , 'sess , S > { type Target = SharedContext < 'p , 'sess , S > ; fn deref (& self) -> & Self :: Target { & self . shared } }
    };
}

impl_280!();