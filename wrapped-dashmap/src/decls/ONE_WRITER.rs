macro_rules! ONE_WRITER {
    () => {
        const ONE_WRITER : usize = ! (READERS_PARKED | WRITERS_PARKED) ;
    };
}

ONE_WRITER!()