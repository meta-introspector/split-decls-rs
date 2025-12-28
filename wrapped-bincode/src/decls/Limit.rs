macro_rules! Limit {
    () => {
        # [doc = " Sets the byte limit to N."] # [derive (Copy , Clone , Debug)] pub struct Limit < const N : usize > ;
    };
}

Limit!()