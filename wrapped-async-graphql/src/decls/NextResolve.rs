macro_rules! deps {
    () => {
        Extension!();
        ResolveFut!();
    };
}

macro_rules! NextResolve {
    () => {
        deps!();
        # [doc = " The remainder of a extension chain for resolve."] pub struct NextResolve < 'a > { chain : & 'a [Arc < dyn Extension >] , resolve_fut : ResolveFut < 'a > , }
    };
}

NextResolve!();