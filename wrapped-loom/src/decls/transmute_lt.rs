macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! transmute_lt {
    () => {
        deps!();
        unsafe fn transmute_lt < 'a , 'b > (state : & 'a RefCell < State < 'b > >) -> & 'a RefCell < State < 'static > > { :: std :: mem :: transmute (state) }
    };
}

transmute_lt!()