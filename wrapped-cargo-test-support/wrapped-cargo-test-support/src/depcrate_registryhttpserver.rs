// Generated macro for HttpServer (struct)
macro_rules! Depcrate_registryHttpServer {
() => {
// Module: crate::registry
// Provides: {"HttpServer"}
// Dependencies: {}
pub struct HttpServer { listener : TcpListener , registry_path : PathBuf , dl_path : PathBuf , api_path : PathBuf , addr : SocketAddr , token : Token , auth_required : bool , custom_responders : HashMap < String , RequestCallback > , not_found_handler : RequestCallback , delayed_index_update : usize , }
};
}
