macro_rules! deps {
    () => {
        SharedContext!();
        Stage!();
        AttributeParser!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < 'p , 'sess : 'p , S : Stage > Deref for SharedContext < 'p , 'sess , S > { type Target = AttributeParser < 'sess , S > ; fn deref (& self) -> & Self :: Target { self . cx } }
    };
}

impl_282!();