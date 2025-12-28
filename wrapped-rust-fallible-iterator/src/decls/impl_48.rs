macro_rules! deps {
    () => {
        FromFn!();
        FallibleIterator!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < I , E , F > FallibleIterator for FromFn < F > where F : FnMut () -> Result < Option < I > , E > , { type Item = I ; type Error = E ; fn next (& mut self) -> Result < Option < I > , E > { (self . fun) () } }
    };
}

impl_48!()