macro_rules! deps {
    () => {
        StockVectorViewIterator!();
        IIterable_Impl!();
        IIterator!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < T > IIterable_Impl < T > for StockVectorView_Impl < T > where T : RuntimeType , T :: Default : Clone + PartialEq , { fn First (& self) -> Result < IIterator < T > > { Ok (ComObject :: new (StockVectorViewIterator { owner : self . to_object () , current : 0 . into () , }) . into_interface ()) } }
    };
}

impl_103!();