macro_rules! deps {
    () => {
        Var!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a > Var < 'a > { fn new < E > (name : E) -> Self where E : Into < Cow < 'a , str > > , { Var { name : name . into () , default : None , } } fn new_with_default < E , V > (name : E , default : V) -> Self where E : Into < Cow < 'a , str > > , V : Into < Cow < 'a , str > > , { Var { name : name . into () , default : Some (default . into ()) , } } fn get (& self) -> Option < String > { env :: var (& * self . name) . ok () . or_else (| | self . default . clone () . map (| v | v . into_owned ())) } }
    };
}

impl_14!()