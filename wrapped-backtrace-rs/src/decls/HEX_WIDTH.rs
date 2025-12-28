macro_rules! HEX_WIDTH {
    () => {
        const HEX_WIDTH : usize = 2 + 2 * core :: mem :: size_of :: < usize > () ;
    };
}

HEX_WIDTH!();