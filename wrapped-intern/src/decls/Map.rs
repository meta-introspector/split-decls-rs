macro_rules! deps {
    () => {
        InternMap!();
        Symbol!();
    };
}

macro_rules! Map {
    () => {
        deps!();
        type Map = InternMap < Symbol > ;
    };
}

Map!()