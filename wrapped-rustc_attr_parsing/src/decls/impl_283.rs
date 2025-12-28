macro_rules! deps {
    () => {
        SharedContext!();
        Stage!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < 'p , 'sess : 'p , S : Stage > DerefMut for SharedContext < 'p , 'sess , S > { fn deref_mut (& mut self) -> & mut Self :: Target { self . cx } }
    };
}

impl_283!()