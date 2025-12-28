macro_rules! deps {
    () => {
        ParseResultBase!();
    };
}

macro_rules! ParseResultExt {
    () => {
        deps!();
        pub trait ParseResultExt < T > : Sized { fn map < U , F > (self , op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static ; }
    };
}

ParseResultExt!();