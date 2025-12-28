macro_rules! deps {
    () => {
        UnzipOp!();
    };
}

macro_rules! Unzip {
    () => {
        deps!();
        # [doc = " An `UnzipOp` that splits a tuple directly into the two consumers."] struct Unzip ;
    };
}

Unzip!()