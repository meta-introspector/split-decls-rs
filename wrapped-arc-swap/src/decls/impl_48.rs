macro_rules! deps {
    () => {
        Strategy!();
        RefCnt!();
        MapCache!();
        Access!();
        ArcSwapAny!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < A , T , S , F , U > Access < U > for MapCache < A , T , F > where A : Deref < Target = ArcSwapAny < T , S > > , T : RefCnt , S : Strategy < T > , F : FnMut (& T) -> & U , { fn load (& mut self) -> & U { (self . projection) (self . inner . load ()) } }
    };
}

impl_48!();