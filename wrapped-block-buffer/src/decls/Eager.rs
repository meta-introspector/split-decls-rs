macro_rules! Eager {
    () => {
        # [doc = " Eager block buffer kind, which guarantees that buffer position"] # [doc = " always lies in the range of `0..BlockSize`."] # [derive (Copy , Clone , Debug , Default)] pub struct Eager { }
    };
}

Eager!()