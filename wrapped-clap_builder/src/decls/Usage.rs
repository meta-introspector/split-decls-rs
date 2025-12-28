macro_rules! deps {
    () => {
        Styles!();
        ChildGraph!();
        Id!();
        Command!();
    };
}

macro_rules! Usage {
    () => {
        deps!();
        pub (crate) struct Usage < 'cmd > { cmd : & 'cmd Command , styles : & 'cmd Styles , required : Option < & 'cmd ChildGraph < Id > > , }
    };
}

Usage!();