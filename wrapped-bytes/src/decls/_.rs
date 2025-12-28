macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        const _ : [() ; 0 - mem :: align_of :: < Shared > () % 2] = [] ;
    };
}

_!()