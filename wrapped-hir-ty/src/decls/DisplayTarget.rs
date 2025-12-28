macro_rules! DisplayTarget {
    () => {
        # [derive (Debug , Clone , Copy)] pub struct DisplayTarget { krate : Crate , pub edition : Edition , }
    };
}

DisplayTarget!()