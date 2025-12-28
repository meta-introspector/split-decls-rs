macro_rules! deps {
    () => {
        Fmt!();
        Source!();
        From!();
        Transparent!();
        Display!();
    };
}

macro_rules! Attrs {
    () => {
        deps!();
        pub struct Attrs < 'a > { pub display : Option < Display < 'a > > , pub source : Option < Source < 'a > > , pub backtrace : Option < & 'a Attribute > , pub from : Option < From < 'a > > , pub transparent : Option < Transparent < 'a > > , pub fmt : Option < Fmt < 'a > > , }
    };
}

Attrs!()