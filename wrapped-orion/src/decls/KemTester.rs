macro_rules! KemTester {
    () => {
        pub struct KemTester < T , K , C > { _kem : PhantomData < T > , _return_type_k : PhantomData < K > , _return_type_c : PhantomData < C > , }
    };
}

KemTester!();