macro_rules! deps {
    () => {
        Offset!();
        Input!();
        Error!();
        PResult!();
        Err!();
        Parser!();
        Consumed!();
        OutputMode!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Consumed < F > where I : Clone + Offset + Input , F : Parser < I > , { type Output = (I , < F as Parser < I > > :: Output) ; type Error = < F as Parser < I > > :: Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OM > (i) { Ok ((remaining , result)) => { let index = input . offset (& remaining) ; Ok ((remaining , OM :: Output :: map (result , | res | { let consumed = input . take (index) ; (consumed , res) }) ,)) } Err (e) => Err (e) , } } }
    };
}

impl_125!()