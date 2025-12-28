macro_rules! deps {
    () => {
        Event!();
        Section!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl Section < '_ > { # [doc = " Turn this instance into a fully owned one with `'static` lifetime."] # [must_use] pub fn to_owned (& self) -> Section < 'static > { Section { header : self . header . to_owned () , events : self . events . iter () . map (Event :: to_owned) . collect () , } } }
    };
}

impl_174!()