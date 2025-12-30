// Generated macro for connect (function)
macro_rules! Depcrate_netconnect {
() => {
// Module: crate::net
// Provides: {"connect"}
// Dependencies: {}
# [cfg (any (feature = "async-client" , feature = "blocking-client"))] # [gix :: protocol :: maybe_async :: maybe_async] pub async fn connect < Url , E > (url : Url , options : io_mode :: connect :: Options ,) -> Result < gix :: protocol :: SendFlushOnDrop < Box < dyn io_mode :: Transport + Send > > , io_mode :: connect :: Error > where Url : TryInto < gix :: url :: Url , Error = E > , gix :: url :: parse :: Error : From < E > , { Ok (gix :: protocol :: SendFlushOnDrop :: new (io_mode :: connect :: connect (url , options) . await ? , false ,)) }
};
}
