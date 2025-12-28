macro_rules! deps {
    () => {
        StockMapView!();
        IMapView!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < K , V > From < std :: collections :: BTreeMap < K :: Default , V :: Default > > for IMapView < K , V > where K : RuntimeType , V : RuntimeType , K :: Default : Clone + Ord , V :: Default : Clone , { fn from (map : std :: collections :: BTreeMap < K :: Default , V :: Default >) -> Self { StockMapView { map } . into () } }
    };
}

impl_100!()