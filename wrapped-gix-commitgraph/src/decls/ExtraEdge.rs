macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! ExtraEdge {
    () => {
        deps!();
        enum ExtraEdge { Internal (Position) , Last (Position) , }
    };
}

ExtraEdge!()