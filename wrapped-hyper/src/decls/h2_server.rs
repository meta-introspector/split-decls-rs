macro_rules! deps {
    () => {
        Executor!();
    };
}

macro_rules! h2_server {
    () => {
        deps!();
        # [cfg (all (feature = "server" , feature = "http2"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "server" , feature = "http2"))))] mod h2_server { use crate :: { proto :: h2 :: server :: H2Stream , rt :: Executor } ; use http_body :: Body ; use std :: future :: Future ; # [doc = " An executor to spawn http2 connections."] # [doc = ""] # [doc = " This trait is implemented for any type that implements [`Executor`]"] # [doc = " trait for any future."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside this crate."] # [doc = ""] # [doc = " [`Executor`]: crate::rt::Executor"] pub trait Http2ServerConnExec < F , B : Body > : super :: Http2UpgradedExec < B :: Data > + sealed :: Sealed < (F , B) > + Clone { # [doc (hidden)] fn execute_h2stream (& mut self , fut : H2Stream < F , B , Self >) ; } # [doc (hidden)] impl < E , F , B > Http2ServerConnExec < F , B > for E where E : Clone , E : Executor < H2Stream < F , B , E > > , E : super :: Http2UpgradedExec < B :: Data > , H2Stream < F , B , E > : Future < Output = () > , B : Body , { fn execute_h2stream (& mut self , fut : H2Stream < F , B , E >) { self . execute (fut) } } impl < E , F , B > sealed :: Sealed < (F , B) > for E where E : Clone , E : Executor < H2Stream < F , B , E > > , E : super :: Http2UpgradedExec < B :: Data > , H2Stream < F , B , E > : Future < Output = () > , B : Body , { } mod sealed { pub trait Sealed < T > { } } }
    };
}

h2_server!();