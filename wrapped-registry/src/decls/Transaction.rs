macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! Transaction {
    () => {
        deps!();
        # [doc = " A transaction object."] # [repr (transparent)] # [derive (Debug)] pub struct Transaction (pub (crate) HANDLE) ;
    };
}

Transaction!();