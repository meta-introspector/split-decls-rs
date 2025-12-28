macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! ReadOnlyView {
    () => {
        deps!();
        # [doc = " A read-only view into a `DashMap`. Allows to obtain raw references to the stored values."] pub struct ReadOnlyView < K , V , S = RandomState > { pub (crate) map : DashMap < K , V , S > , }
    };
}

ReadOnlyView!()