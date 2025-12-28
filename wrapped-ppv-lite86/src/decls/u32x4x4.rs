macro_rules! deps {
    () => {
        StoreBytes!();
        MultiLane!();
        Vector!();
        Vec4Ext!();
        Store!();
        Vec4!();
        BitOps32!();
        Machine!();
        LaneWords4!();
        ArithOps!();
    };
}

macro_rules! u32x4x4 {
    () => {
        deps!();
        pub trait u32x4x4 < M : Machine > : BitOps32 + Store < vec512_storage > + Vec4 < M :: u32x4 > + Vec4Ext < M :: u32x4 > + Vector < [u32 ; 16] > + MultiLane < [M :: u32x4 ; 4] > + ArithOps + LaneWords4 + Into < vec512_storage > + StoreBytes { }
    };
}

u32x4x4!()