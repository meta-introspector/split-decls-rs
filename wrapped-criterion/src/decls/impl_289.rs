macro_rules! deps {
    () => {
        Function!();
        Measurement!();
        Bencher!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < M : Measurement , F , T > Function < M , F , T > where F : FnMut (& mut Bencher < '_ , M > , & T) , T : ? Sized , { pub fn new (f : F) -> Function < M , F , T > { Function { f , _phantom : PhantomData , _phamtom2 : PhantomData , } } }
    };
}

impl_289!();