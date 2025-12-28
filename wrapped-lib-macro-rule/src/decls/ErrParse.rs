macro_rules! deps {
    () => {
        Failure!();
    };
}

macro_rules! ErrParse {
    () => {
        deps!();
        # [doc = " Failure variant with collected diagnostics."] pub struct ErrParse < T > { pub failures : Vec < Failure > , _phantom : std :: marker :: PhantomData < T > , }
    };
}

ErrParse!();