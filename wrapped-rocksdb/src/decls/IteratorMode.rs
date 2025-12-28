macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! IteratorMode {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub enum IteratorMode < 'a > { Start , End , From (& 'a [u8] , Direction) , }
    };
}

IteratorMode!()