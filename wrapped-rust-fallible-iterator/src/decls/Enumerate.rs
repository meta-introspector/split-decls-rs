macro_rules! Enumerate {
    () => {
        # [doc = " An iterator that yields the iteration count as well as the values of the"] # [doc = " underlying iterator."] # [derive (Clone , Debug)] pub struct Enumerate < I > { it : I , n : usize , }
    };
}

Enumerate!();