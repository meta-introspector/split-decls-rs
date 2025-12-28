macro_rules! deps {
    () => {
        AcceptContext!();
        AlignParser!();
        Stage!();
        ArgParser!();
        AlignStaticParser!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl AlignStaticParser { const PATH : & 'static [Symbol] = & [sym :: rustc_align_static] ; const TEMPLATE : AttributeTemplate = AlignParser :: TEMPLATE ; fn parse < 'c , S : Stage > (& mut self , cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) { self . 0 . parse (cx , args) } }
    };
}

impl_169!();