macro_rules! deps {
    () => {
        Response!();
    };
}

macro_rules! RequestFut {
    () => {
        deps!();
        type RequestFut < 'a > = & 'a mut (dyn Future < Output = Response > + Send + Unpin) ;
    };
}

RequestFut!()