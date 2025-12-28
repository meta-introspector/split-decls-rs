macro_rules! deps {
    () => {
        Plotter!();
    };
}

macro_rules! Html {
    () => {
        deps!();
        pub struct Html { templates : TinyTemplate < 'static > , plotter : RefCell < Box < dyn Plotter > > , }
    };
}

Html!()