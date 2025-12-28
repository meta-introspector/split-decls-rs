macro_rules! Zip {
    () => {
        # [doc = " See [`multizip`] for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Zip < T > { t : T , }
    };
}

Zip!();