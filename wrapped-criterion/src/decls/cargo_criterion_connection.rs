macro_rules! cargo_criterion_connection {
    () => {
        fn cargo_criterion_connection () -> & 'static Option < Mutex < Connection > > { static CARGO_CRITERION_CONNECTION : OnceLock < Option < Mutex < Connection > > > = OnceLock :: new () ; CARGO_CRITERION_CONNECTION . get_or_init (| | match std :: env :: var ("CARGO_CRITERION_PORT") { Ok (port_str) => { let port : u16 = port_str . parse () . ok () ? ; let stream = TcpStream :: connect (("localhost" , port)) . ok () ? ; Some (Mutex :: new (Connection :: new (stream) . ok () ?)) } Err (_) => None , }) }
    };
}

cargo_criterion_connection!()