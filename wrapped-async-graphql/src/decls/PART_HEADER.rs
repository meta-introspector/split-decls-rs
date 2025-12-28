macro_rules! PART_HEADER {
    () => {
        static PART_HEADER : Bytes = Bytes :: from_static (b"--graphql\r\nContent-Type: application/json\r\n\r\n") ;
    };
}

PART_HEADER!()