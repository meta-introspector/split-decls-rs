macro_rules! deps {
    () => {
        FluentNumber!();
        FluentNumberOptions!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl FromStr for FluentNumber { type Err = std :: num :: ParseFloatError ; fn from_str (input : & str) -> Result < Self , Self :: Err > { f64 :: from_str (input) . map (| n | { let mfd = input . find ('.') . map (| pos | input . len () - pos - 1) ; let opts = FluentNumberOptions { minimum_fraction_digits : mfd , .. Default :: default () } ; Self :: new (n , opts) }) } }
    };
}

impl_80!()