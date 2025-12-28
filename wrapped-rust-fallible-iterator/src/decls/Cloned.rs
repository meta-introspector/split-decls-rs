macro_rules! Cloned {
    () => {
        # [doc = " An iterator which clones the elements of the underlying iterator."] # [derive (Clone , Debug)] pub struct Cloned < I > (I) ;
    };
}

Cloned!();