macro_rules! deps {
    () => {
        ServerResult!();
    };
}

macro_rules! ResolveFut {
    () => {
        deps!();
        # [doc = " A future type used to resolve the field"] pub type ResolveFut < 'a > = & 'a mut (dyn Future < Output = ServerResult < Option < Value > > > + Send + Unpin) ;
    };
}

ResolveFut!();