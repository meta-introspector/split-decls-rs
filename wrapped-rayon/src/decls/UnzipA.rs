macro_rules! deps {
    () => {
        Consumer!();
    };
}

macro_rules! UnzipA {
    () => {
        deps!();
        # [doc = " A fake iterator to intercept the `Consumer` for type `A`."] struct UnzipA < 'b , I , OP , FromB > { base : I , op : OP , b : & 'b mut FromB , }
    };
}

UnzipA!();