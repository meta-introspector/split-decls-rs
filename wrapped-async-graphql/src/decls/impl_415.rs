macro_rules! deps {
    () => {
        Directive!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        impl Directive { # [doc = " Create a directive usage"] pub fn new (name : impl Into < String >) -> Self { Self { name : name . into () , args : IndexMap :: default () , } } # [doc = " Add an argument to the directive"] # [inline] pub fn argument (mut self , name : impl Into < String > , value : Value) -> Self { self . args . insert (name . into () , value) ; self } }
    };
}

impl_415!();