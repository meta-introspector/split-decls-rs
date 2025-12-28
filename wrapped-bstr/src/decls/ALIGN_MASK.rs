macro_rules! ALIGN_MASK {
    () => {
        const ALIGN_MASK : usize = core :: mem :: align_of :: < usize > () - 1 ;
    };
}

ALIGN_MASK!()