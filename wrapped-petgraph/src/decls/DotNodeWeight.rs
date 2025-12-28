macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! DotNodeWeight {
    () => {
        deps!();
        pub type DotNodeWeight < 'a > = Node < (& 'a str , & 'a str) > ;
    };
}

DotNodeWeight!()