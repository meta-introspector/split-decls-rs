macro_rules! deps {
    () => {
        WnafScalar!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < F : PrimeField , const WINDOW_SIZE : usize > WnafScalar < F , WINDOW_SIZE > { # [doc = " Computes the w-NAF representation of the given scalar with the specified"] # [doc = " `WINDOW_SIZE`."] pub fn new (scalar : & F) -> Self { let mut wnaf = vec ! [] ; wnaf_form (& mut wnaf , scalar . to_repr () , WINDOW_SIZE) ; WnafScalar { wnaf , field : PhantomData :: default () , } } }
    };
}

impl_37!();