macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! RefTracking {
    () => {
        deps!();
        # [doc = " State for tracking recursive validation of references"] pub struct RefTracking < T , PATH = () > { seen : FxHashSet < T > , todo : Vec < (T , PATH) > , }
    };
}

RefTracking!();