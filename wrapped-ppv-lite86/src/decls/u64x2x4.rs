macro_rules! deps {
    () => {
        MultiLane!();
        BitOps64!();
        Vec4!();
        Machine!();
        Store!();
        ArithOps!();
    };
}

macro_rules! u64x2x4 {
    () => {
        deps!();
        pub trait u64x2x4 < M : Machine > : BitOps64 + Store < vec512_storage > + Vec4 < M :: u64x2 > + MultiLane < [M :: u64x2 ; 4] > + ArithOps + Into < vec512_storage > { }
    };
}

u64x2x4!();