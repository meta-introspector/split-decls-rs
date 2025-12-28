macro_rules! IteratorItem {
    () => {
        pub struct IteratorItem < T > { value : T , pub is_first : bool , pub is_last : bool , }
    };
}

IteratorItem!()