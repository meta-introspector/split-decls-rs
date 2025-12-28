macro_rules! deps {
    () => {
        StockIterator!();
        IIterable_Impl!();
        IIterator!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T > IIterable_Impl < T > for StockIterable_Impl < T > where T : RuntimeType , T :: Default : Clone , { fn First (& self) -> Result < IIterator < T > > { Ok (ComObject :: new (StockIterator { owner : self . to_object () , current : 0 . into () , }) . into_interface ()) } }
    };
}

impl_88!()