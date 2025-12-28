macro_rules! deps {
    () => {
        BitOps128!();
        MultiLane!();
        Swap64!();
        Store!();
        Machine!();
        Vec4!();
    };
}

macro_rules! u128x4 {
    () => {
        deps!();
        pub trait u128x4 < M : Machine > : BitOps128 + Store < vec512_storage > + Vec4 < M :: u128x1 > + MultiLane < [M :: u128x1 ; 4] > + Swap64 + Into < vec512_storage > { }
    };
}

u128x4!();