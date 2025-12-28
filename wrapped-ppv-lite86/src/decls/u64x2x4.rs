macro_rules! deps {
    () => {
        Machine!();
        ArithOps!();
        Store!();
        MultiLane!();
        Vec4!();
        BitOps64!();
    };
}

macro_rules! u64x2x4 {
    () => {
        deps!();
        pub trait u64x2x4 < M : Machine > : BitOps64 + Store < vec512_storage > + Vec4 < M :: u64x2 > + MultiLane < [M :: u64x2 ; 4] > + ArithOps + Into < vec512_storage > { }
    };
}

u64x2x4!()