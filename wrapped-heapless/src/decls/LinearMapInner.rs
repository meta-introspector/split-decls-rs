macro_rules! deps {
    () => {
        LinearMapView!();
        VecInner!();
        LinearMap!();
    };
}

macro_rules! LinearMapInner {
    () => {
        deps!();
        # [doc = " Base struct for [`LinearMap`] and [`LinearMapView`]"] # [cfg_attr (feature = "zeroize" , derive (Zeroize) , zeroize (bound = "S: Zeroize, K: Zeroize, V: Zeroize"))] pub struct LinearMapInner < K , V , S : LinearMapStorage < K , V > + ? Sized > { pub (crate) buffer : VecInner < (K , V) , usize , S > , }
    };
}

LinearMapInner!();