macro_rules! Iterator {
    () => {
        # [doc = " A normal (non-fallible) iterator which wraps a fallible iterator."] # [derive (Clone , Debug)] pub struct Iterator < I > (I) ;
    };
}

Iterator!();