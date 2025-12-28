macro_rules! deps {
    () => {
        Error!();
        PResult!();
        Parser!();
        OutputMode!();
        Cond!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < I , F > Parser < I > for Cond < F > where F : Parser < I > , { type Output = Option < < F as Parser < I > > :: Output > ; type Error = < F as Parser < I > > :: Error ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { match & mut self . parser { None => Ok ((input , OM :: Output :: bind (| | None))) , Some (f) => f . process :: < OM > (input) . map (| (i , o) | (i , OM :: Output :: map (o , Some))) , } } }
    };
}

impl_102!()