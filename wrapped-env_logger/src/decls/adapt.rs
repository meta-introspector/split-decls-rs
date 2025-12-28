macro_rules! deps {
    () => {
        WriteStyle!();
    };
}

macro_rules! adapt {
    () => {
        deps!();
        # [cfg (feature = "color")] fn adapt (buf : & [u8] , write_style : WriteStyle) -> io :: Result < Vec < u8 > > { use std :: io :: Write as _ ; let adapted = Vec :: with_capacity (buf . len ()) ; let mut stream = anstream :: AutoStream :: new (adapted , write_style . into ()) ; stream . write_all (buf) ? ; let adapted = stream . into_inner () ; Ok (adapted) }
    };
}

adapt!();