macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < W > Clone for Write < W > where W : Clone , { fn clone (& self) -> Self { Write { compressor : impls :: new_compress () , inner : self . inner . clone () , buf : self . buf , } } }
    };
}

impl_100!();