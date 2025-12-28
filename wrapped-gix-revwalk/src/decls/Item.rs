macro_rules! Item {
    () => {
        pub (crate) struct Item < K , T > { key : K , value : T , }
    };
}

Item!();