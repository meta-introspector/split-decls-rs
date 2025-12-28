macro_rules! Gb18030Pending {
    () => {
        enum Gb18030Pending { None , One (u8) , Two (u8 , u8) , Three (u8 , u8 , u8) , }
    };
}

Gb18030Pending!();