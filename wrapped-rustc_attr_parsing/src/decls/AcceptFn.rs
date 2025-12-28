macro_rules! deps {
    () => {
        ArgParser!();
        AcceptContext!();
    };
}

macro_rules! AcceptFn {
    () => {
        deps!();
        type AcceptFn < S > = Box < dyn for < 'sess , 'a > Fn (& mut AcceptContext < '_ , 'sess , S > , & ArgParser < 'a >) + Send + Sync > ;
    };
}

AcceptFn!()