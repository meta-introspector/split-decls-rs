macro_rules! deps {
    () => {
        ParseResultExt!();
        OkParse!();
        ParseResultBase!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T > ParseResultExt < T > for OkParse < T > { fn map < U , F > (self , op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static { Box :: new (OkParse (op (self . 0))) } }
    };
}

impl_18!()