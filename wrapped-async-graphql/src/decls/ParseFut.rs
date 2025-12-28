macro_rules! deps {
    () => {
        ServerResult!();
    };
}

macro_rules! ParseFut {
    () => {
        deps!();
        type ParseFut < 'a > = & 'a mut (dyn Future < Output = ServerResult < ExecutableDocument > > + Send + Unpin) ;
    };
}

ParseFut!();