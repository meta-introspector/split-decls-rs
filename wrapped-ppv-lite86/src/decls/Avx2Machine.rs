macro_rules! Avx2Machine {
    () => {
        # [derive (Copy , Clone)] pub struct Avx2Machine < NI > (PhantomData < NI >) ;
    };
}

Avx2Machine!();