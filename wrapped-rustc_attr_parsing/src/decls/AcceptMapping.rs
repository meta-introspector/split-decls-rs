macro_rules! deps {
    () => {
        AcceptFn!();
    };
}

macro_rules! AcceptMapping {
    () => {
        deps!();
        type AcceptMapping < T , S > = & 'static [(& 'static [Symbol] , AttributeTemplate , AcceptFn < T , S >)] ;
    };
}

AcceptMapping!()