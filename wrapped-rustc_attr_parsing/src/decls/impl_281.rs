macro_rules! deps {
    () => {
        Stage!();
        FinalizeContext!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl < 'p , 'sess : 'p , S : Stage > DerefMut for FinalizeContext < 'p , 'sess , S > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . shared } }
    };
}

impl_281!()