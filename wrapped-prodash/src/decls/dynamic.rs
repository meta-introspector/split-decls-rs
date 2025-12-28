macro_rules! deps {
    () => {
        Kind!();
        Unit!();
        DisplayValue!();
    };
}

macro_rules! dynamic {
    () => {
        deps!();
        # [doc = " Returns a unit that is a dynamic `label`."] pub fn dynamic (label : impl DisplayValue + Send + Sync + 'static) -> Unit { Unit { kind : Kind :: Dynamic (Arc :: new (label)) , mode : None , } }
    };
}

dynamic!()