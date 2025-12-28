macro_rules! deps {
    () => {
        IKeyValuePair!();
        IIterator!();
        StockMapViewIterator!();
        IIterable_Impl!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < K , V > IIterable_Impl < IKeyValuePair < K , V > > for StockMapView_Impl < K , V > where K : RuntimeType , V : RuntimeType , K :: Default : Clone + Ord , V :: Default : Clone , { fn First (& self) -> Result < IIterator < IKeyValuePair < K , V > > > { Ok (ComObject :: new (StockMapViewIterator :: < K , V > { _owner : self . to_object () , current : std :: sync :: RwLock :: new (self . map . iter ()) , }) . into_interface ()) } }
    };
}

impl_94!();