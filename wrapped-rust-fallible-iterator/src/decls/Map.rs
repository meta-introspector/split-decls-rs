macro_rules! Map {
    () => {
        # [doc = " An iterator which applies a fallible transform to the elements of the"] # [doc = " underlying iterator."] # [derive (Clone)] pub struct Map < T , F > { it : T , f : F , }
    };
}

Map!();