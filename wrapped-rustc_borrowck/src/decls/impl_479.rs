macro_rules! deps {
    () => {
        Locations!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        impl Locations { pub fn from_location (& self) -> Option < Location > { match self { Locations :: All (_) => None , Locations :: Single (from_location) => Some (* from_location) , } } # [doc = " Gets a span representing the location."] pub fn span (& self , body : & Body < '_ >) -> Span { match self { Locations :: All (span) => * span , Locations :: Single (l) => body . source_info (* l) . span , } } }
    };
}

impl_479!()