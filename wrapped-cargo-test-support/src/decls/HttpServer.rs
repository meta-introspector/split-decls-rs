macro_rules! deps {
    () => {
        Token!();
        RequestCallback!();
    };
}

macro_rules! HttpServer {
    () => {
        deps!();
        pub struct HttpServer { listener : TcpListener , registry_path : PathBuf , dl_path : PathBuf , api_path : PathBuf , addr : SocketAddr , token : Token , auth_required : bool , custom_responders : HashMap < String , RequestCallback > , not_found_handler : RequestCallback , delayed_index_update : usize , }
    };
}

HttpServer!()