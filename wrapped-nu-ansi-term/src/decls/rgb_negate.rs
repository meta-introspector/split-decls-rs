macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! rgb_negate {
    () => {
        deps!();
        const fn rgb_negate (rgb : & Rgb) -> Rgb { Rgb :: new (255 - rgb . r , 255 - rgb . g , 255 - rgb . b) }
    };
}

rgb_negate!();