macro_rules! deps {
    () => {
        CVWords!();
    };
}

macro_rules! IV {
    () => {
        deps!();
        const IV : & CVWords = & [0x6A09E667 , 0xBB67AE85 , 0x3C6EF372 , 0xA54FF53A , 0x510E527F , 0x9B05688C , 0x1F83D9AB , 0x5BE0CD19 ,] ;
    };
}

IV!()