// Generated macro for Connection (struct)
macro_rules! Depcrate_proto_connectionConnection {
() => {
// Module: crate::proto::connection
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " An H2 connection"] # [derive (Debug)] pub (crate) struct Connection < T , P , B : Buf = Bytes > where P : Peer , { # [doc = " Read / write frame values"] codec : Codec < T , Prioritized < B > > , inner : ConnectionInner < P , B > , }
};
}
