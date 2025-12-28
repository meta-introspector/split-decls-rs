macro_rules! deps {
    () => {
        Callbacks!();
    };
}

macro_rules! EasyData {
    () => {
        deps!();
        pub struct EasyData { running : Cell < bool > , owned : Callbacks < 'static > , borrowed : Cell < * mut Callbacks < 'static > > , }
    };
}

EasyData!();