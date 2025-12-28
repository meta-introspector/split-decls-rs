macro_rules! Holes {
    () => {
        # [doc = " Holes are the node indices of vacancies, with known length"] struct Holes < T > (usize , T) ;
    };
}

Holes!();