macro_rules! IntoIter {
    () => {
        # [doc = " Parallel iterator that moves out of a vector."] # [derive (Debug , Clone)] pub struct IntoIter < T > { vec : Vec < T > , }
    };
}

IntoIter!()