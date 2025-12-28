macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! DIRECTIONS {
    () => {
        deps!();
        const DIRECTIONS : [Direction ; 2] = [Outgoing , Incoming] ;
    };
}

DIRECTIONS!()