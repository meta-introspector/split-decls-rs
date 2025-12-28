macro_rules! deps {
    () => {
        Recv!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < St : ? Sized + Unpin > Unpin for Recv < '_ , St > { }
    };
}

impl_81!()