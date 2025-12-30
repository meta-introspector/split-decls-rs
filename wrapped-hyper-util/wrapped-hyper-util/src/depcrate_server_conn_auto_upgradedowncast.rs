// Generated macro for downcast (function)
macro_rules! Depcrate_server_conn_auto_upgradedowncast {
() => {
// Module: crate::server::conn::auto::upgrade
// Provides: {"downcast"}
// Dependencies: {}
# [doc = " Tries to downcast the internal trait object to the type passed."] # [doc = ""] # [doc = " On success, returns the downcasted parts. On error, returns the Upgraded back."] # [doc = " This is a kludge to work around the fact that the machinery provided by"] # [doc = " [`hyper_util::server::conn::auto`] wraps the inner `T` with a private type"] # [doc = " that is not reachable from outside the crate."] # [doc = ""] # [doc = " [`hyper_util::server::conn::auto`]: crate::server::conn::auto"] # [doc = ""] # [doc = " This kludge will be removed when this machinery is added back to the main"] # [doc = " `hyper` code."] pub fn downcast < T > (upgraded : Upgraded) -> Result < Parts < T > , Upgraded > where T : Read + Write + Unpin + 'static , { let hyper :: upgrade :: Parts { io : rewind , mut read_buf , .. } = upgraded . downcast :: < Rewind < T > > () ? ; if let Some (pre) = rewind . pre { read_buf = if read_buf . is_empty () { pre } else { let mut buf = BytesMut :: from (read_buf) ; buf . extend_from_slice (& pre) ; buf . freeze () } ; } Ok (Parts { io : rewind . inner , read_buf , }) }
};
}
