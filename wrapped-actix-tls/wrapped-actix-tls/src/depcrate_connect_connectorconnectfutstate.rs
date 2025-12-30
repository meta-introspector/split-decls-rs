// Generated macro for ConnectFutState (enum)
macro_rules! Depcrate_connect_connectorConnectFutState {
() => {
// Module: crate::connect::connector
// Provides: {"ConnectFutState"}
// Dependencies: {}
# [doc = " Container for the intermediate states of [`ConnectFut`]."] pub (crate) enum ConnectFutState < R : Host > { Resolved (ConnectInfo < R >) , Connected (Connection < R , TcpStream >) , }
};
}
