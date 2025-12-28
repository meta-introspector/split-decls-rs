macro_rules! EOF {
    () => {
        static EOF : Bytes = Bytes :: from_static (b"--graphql--\r\n") ;
    };
}

EOF!()