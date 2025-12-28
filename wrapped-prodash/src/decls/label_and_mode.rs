macro_rules! deps {
    () => {
        Unit!();
        Kind!();
        Mode!();
    };
}

macro_rules! label_and_mode {
    () => {
        deps!();
        # [doc = " Returns a unit that is a static `label` along with information on where to display a fraction and throughput."] pub fn label_and_mode (label : & 'static str , mode : display :: Mode) -> Unit { Unit { kind : Kind :: Label (label) , mode : Some (mode) , } }
    };
}

label_and_mode!();