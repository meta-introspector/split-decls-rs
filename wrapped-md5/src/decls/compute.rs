macro_rules! deps {
    () => {
        Digest!();
        Context!();
    };
}

macro_rules! compute {
    () => {
        deps!();
        # [doc = " Compute the digest of data."] # [inline] pub fn compute < T : AsRef < [u8] > > (data : T) -> Digest { let mut context = Context :: new () ; context . consume (data) ; context . finalize () }
    };
}

compute!();