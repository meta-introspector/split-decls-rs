macro_rules! deps {
    () => {
        GroupingMapFn!();
        MapSpecialCaseFn!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < V , K , F : FnMut (& V) -> K > MapSpecialCaseFn < V > for GroupingMapFn < F > { type Out = (K , V) ; fn call (& mut self , v : V) -> Self :: Out { ((self . 0) (& v) , v) } }
    };
}

impl_289!()