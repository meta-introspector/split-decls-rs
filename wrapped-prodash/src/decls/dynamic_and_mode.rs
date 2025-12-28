macro_rules! deps {
    () => {
        Kind!();
        Unit!();
        Mode!();
        DisplayValue!();
    };
}

macro_rules! dynamic_and_mode {
    () => {
        deps!();
        # [doc = " Returns a unit that is a dynamic `label` along with information on where to display a fraction and throughput."] pub fn dynamic_and_mode (label : impl DisplayValue + Send + Sync + 'static , mode : display :: Mode) -> Unit { Unit { kind : Kind :: Dynamic (Arc :: new (label)) , mode : Some (mode) , } }
    };
}

dynamic_and_mode!();