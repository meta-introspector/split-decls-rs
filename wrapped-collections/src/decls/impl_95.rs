macro_rules! deps {
    () => {
        IMapView!();
        IMapView_Impl!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < K , V > IMapView_Impl < K , V > for StockMapView_Impl < K , V > where K : RuntimeType , V : RuntimeType , K :: Default : Clone + Ord , V :: Default : Clone , { fn Lookup (& self , key : Ref < K >) -> Result < V > { let value = self . map . get (& * key) . ok_or_else (| | Error :: from (E_BOUNDS)) ? ; V :: from_default (value) } fn Size (& self) -> Result < u32 > { Ok (self . map . len () . try_into () ?) } fn HasKey (& self , key : Ref < K >) -> Result < bool > { Ok (self . map . contains_key (& * key)) } fn Split (& self , first : OutRef < IMapView < K , V > > , second : OutRef < IMapView < K , V > >) -> Result < () > { _ = first . write (None) ; _ = second . write (None) ; Ok (()) } }
    };
}

impl_95!()