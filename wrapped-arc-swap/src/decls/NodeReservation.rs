macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! NodeReservation {
    () => {
        deps!();
        pub struct NodeReservation < 'a > (& 'a Node) ;
    };
}

NodeReservation!();