// Generated macro for connect (function)
macro_rules! Depcrate_client_async_clientconnect {
() => {
// Module: crate::client::async_client
// Provides: {"connect"}
// Dependencies: {}
# [doc = " Connect to the socket."] pub async fn connect (args : & H3iConfig , frame_actions : Vec < Action > , close_trigger_frames : Option < CloseTriggerFrames > ,) -> std :: result :: Result < BuildingConnectionSummary , ClientError > { let quic_settings = create_config (args) ; let mut connection_params = ConnectionParams :: new_client (quic_settings , None , Hooks :: default ()) ; connection_params . session = args . session . clone () ; let ParsedArgs { connect_url , bind_addr , peer_addr , } = parse_args (args) ; let socket = tokio :: net :: UdpSocket :: bind (bind_addr) . await . unwrap () ; socket . connect (peer_addr) . await . unwrap () ; log :: info ! ("connecting to {:} from {:}" , peer_addr , socket . local_addr () . unwrap ()) ; let (h3i , conn_summary_fut) = H3iDriver :: new (frame_actions , close_trigger_frames) ; match tokio_quiche :: quic :: connect_with_config (Socket :: try_from (socket) . unwrap () , connect_url , & connection_params , h3i ,) . await { Ok (_) => Ok (conn_summary_fut) , Err (_) => Err (ClientError :: HandshakeFail) , } }
};
}
