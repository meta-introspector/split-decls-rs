macro_rules! deps {
    () => {
        Test!();
        Seed!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Seed { const MAX_PAD : usize = 34 ; const fn new (needle : & 'static str , haystack : & 'static str , fwd : Option < usize > , rev : Option < usize > ,) -> Seed { Seed { needle , haystack , fwd , rev } } fn generate (self) -> impl Iterator < Item = Test > { assert ! (! self . needle . contains ('#') , "needle must not contain '#'") ; assert ! (! self . haystack . contains ('#') , "haystack must not contain '#'") ; (0 ..= Seed :: MAX_PAD) . map (move | pad | { let needle = self . needle . to_string () ; let prefix = "#" . repeat (pad) ; let haystack = format ! ("{}{}" , prefix , self . haystack) ; let fwd = if needle . is_empty () { Some (0) } else { self . fwd . map (| i | pad + i) } ; let rev = if needle . is_empty () { Some (haystack . len ()) } else { self . rev . map (| i | pad + i) } ; Test { needle , haystack , fwd , rev } }) . chain ((1 ..= Seed :: MAX_PAD) . map (move | pad | { let needle = self . needle . to_string () ; let suffix = "#" . repeat (pad) ; let haystack = format ! ("{}{}" , self . haystack , suffix) ; let fwd = if needle . is_empty () { Some (0) } else { self . fwd } ; let rev = if needle . is_empty () { Some (haystack . len ()) } else { self . rev } ; Test { needle , haystack , fwd , rev } })) } }
    };
}

impl_52!();