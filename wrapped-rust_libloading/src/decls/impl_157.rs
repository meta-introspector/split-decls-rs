macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'lib , T > Clone for Symbol < 'lib , T > { fn clone (& self) -> Symbol < 'lib , T > { Symbol { inner : self . inner . clone () , pd : marker :: PhantomData , } } }
    };
}

impl_157!()