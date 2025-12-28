macro_rules! deps {
    () => {
        Movability!();
    };
}

macro_rules! ClosureKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum ClosureKind { Closure , Coroutine (Movability) , Async , }
    };
}

ClosureKind!()