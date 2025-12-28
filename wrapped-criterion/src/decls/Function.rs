macro_rules! deps {
    () => {
        Measurement!();
        Bencher!();
    };
}

macro_rules! Function {
    () => {
        deps!();
        pub struct Function < M : Measurement , F , T > where F : FnMut (& mut Bencher < '_ , M > , & T) , T : ? Sized , { f : F , _phantom : PhantomData < T > , _phamtom2 : PhantomData < M > , }
    };
}

Function!()