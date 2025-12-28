macro_rules! deps {
    () => {
        Guard!();
        RefCnt!();
        Strategy!();
    };
}

macro_rules! DirectDeref {
    () => {
        deps!();
        # [derive (Debug)] # [doc (hidden)] pub struct DirectDeref < T : RefCnt , S : Strategy < T > > (Guard < T , S >) ;
    };
}

DirectDeref!();