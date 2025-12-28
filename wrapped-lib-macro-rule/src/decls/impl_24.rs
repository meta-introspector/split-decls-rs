macro_rules! deps {
    () => {
        ParseResultExt!();
        ErrParse!();
        ParseResultBase!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T > ParseResultExt < T > for ErrParse < T > { fn map < U , F > (self , _op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static { Box :: new (ErrParse :: new (self . failures)) } }
    };
}

impl_24!()