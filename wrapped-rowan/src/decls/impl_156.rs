macro_rules! deps {
    () => {
        ThinArc!();
        Arc!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < H , T > Drop for ThinArc < H , T > { # [inline] fn drop (& mut self) { let _ = Arc :: from_thin (ThinArc { ptr : self . ptr , phantom : PhantomData }) ; } }
    };
}

impl_156!();