macro_rules! deps {
    () => {
        Result!();
        FieldValue!();
    };
}

macro_rules! BoxResolveFut {
    () => {
        deps!();
        type BoxResolveFut < 'a > = BoxFuture < 'a , Result < BoxStream < 'a , Result < FieldValue < 'a > > > > > ;
    };
}

BoxResolveFut!()