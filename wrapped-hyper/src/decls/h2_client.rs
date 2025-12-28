macro_rules! deps {
    () => {
        Error!();
        Executor!();
        Read!();
        Write!();
    };
}

macro_rules! h2_client {
    () => {
        deps!();
        # [cfg (all (feature = "client" , feature = "http2"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "client" , feature = "http2"))))] mod h2_client { use std :: { error :: Error , future :: Future } ; use crate :: rt :: { Read , Write } ; use crate :: { proto :: h2 :: client :: H2ClientFuture , rt :: Executor } ; # [doc = " An executor to spawn http2 futures for the client."] # [doc = ""] # [doc = " This trait is implemented for any type that implements [`Executor`]"] # [doc = " trait for any future."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside this crate."] # [doc = ""] # [doc = " [`Executor`]: crate::rt::Executor"] pub trait Http2ClientConnExec < B , T > : super :: Http2UpgradedExec < B :: Data > + sealed_client :: Sealed < (B , T) > + Clone where B : http_body :: Body , B :: Error : Into < Box < dyn Error + Send + Sync > > , T : Read + Write + Unpin , { # [doc (hidden)] fn execute_h2_future (& mut self , future : H2ClientFuture < B , T , Self >) ; } impl < E , B , T > Http2ClientConnExec < B , T > for E where E : Clone , E : Executor < H2ClientFuture < B , T , E > > , E : super :: Http2UpgradedExec < B :: Data > , B : http_body :: Body + 'static , B :: Error : Into < Box < dyn Error + Send + Sync > > , H2ClientFuture < B , T , E > : Future < Output = () > , T : Read + Write + Unpin , { fn execute_h2_future (& mut self , future : H2ClientFuture < B , T , E >) { self . execute (future) } } impl < E , B , T > sealed_client :: Sealed < (B , T) > for E where E : Clone , E : Executor < H2ClientFuture < B , T , E > > , E : super :: Http2UpgradedExec < B :: Data > , B : http_body :: Body + 'static , B :: Error : Into < Box < dyn Error + Send + Sync > > , H2ClientFuture < B , T , E > : Future < Output = () > , T : Read + Write + Unpin , { } mod sealed_client { pub trait Sealed < X > { } } }
    };
}

h2_client!();