macro_rules! deps {
    () => {
        OutputMode!();
        OutputM!();
        Input!();
        Err!();
        Error!();
        Parser!();
        PResult!();
        Recognize!();
        Check!();
        Offset!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Recognize < F > where I : Clone + Offset + Input , F : Parser < I > , { type Output = I ; type Error = < F as Parser < I > > :: Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { let i = input . clone () ; match self . parser . process :: < OutputM < Check , OM :: Error , OM :: Incomplete > > (i) { Ok ((i , _)) => { let index = input . offset (& i) ; Ok ((i , OM :: Output :: bind (| | input . take (index)))) } Err (e) => Err (e) , } } }
    };
}

impl_122!();