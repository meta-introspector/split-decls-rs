macro_rules! TOKEN_EXCLUSIVE {
    () => {
        const TOKEN_EXCLUSIVE : ParkToken = ParkToken (WRITER_BIT) ;
    };
}

TOKEN_EXCLUSIVE!()