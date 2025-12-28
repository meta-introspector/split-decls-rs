macro_rules! deps {
    () => {
        ChildGraph!();
        Command!();
        Id!();
        Styles!();
    };
}

macro_rules! Usage {
    () => {
        deps!();
        pub (crate) struct Usage < 'cmd > { cmd : & 'cmd Command , styles : & 'cmd Styles , required : Option < & 'cmd ChildGraph < Id > > , }
    };
}

Usage!()