macro_rules! deps {
    () => {
        Unit!();
        Kind!();
    };
}

macro_rules! label {
    () => {
        deps!();
        # [doc = " Returns a unit that is a static `label`."] pub fn label (label : & 'static str) -> Unit { Unit { kind : Kind :: Label (label) , mode : None , } }
    };
}

label!()