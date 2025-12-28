macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! FnFmt {
    () => {
        deps!();
        # [doc = " Format data using a specific format function"] struct FnFmt < 'a , T , F > (& 'a T , F) ;
    };
}

FnFmt!();