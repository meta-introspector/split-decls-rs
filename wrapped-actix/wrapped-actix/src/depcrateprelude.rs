// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
pub mod prelude { # ! [doc = " The `actix` prelude."] # ! [doc = ""] # ! [doc = " The purpose of this module is to alleviate imports of many common actix"] # ! [doc = " traits by adding a glob import to the top of actix heavy modules:"] # ! [doc = ""] # ! [doc = " ```"] # ! [doc = " # #![allow(unused_imports)]"] # ! [doc = " use actix::prelude::*;"] # ! [doc = " ```"] # [doc (hidden)] # [cfg (feature = "macros")] pub use actix_derive :: { Message , MessageResponse } ; pub use actix_rt :: { Arbiter , ArbiterHandle , System , SystemRunner } ; pub use futures_core :: stream :: Stream ; # [allow (deprecated)] pub use crate :: utils :: Condition ; pub use crate :: { actor :: { Actor , ActorContext , ActorState , AsyncContext , Running , SpawnHandle , Supervised } , actors , address :: { Addr , MailboxError , Recipient , RecipientRequest , Request , SendError } , context :: { Context , ContextFutureSpawner } , dev , fut , fut :: { ActorFuture , ActorFutureExt , ActorStream , ActorStreamExt , ActorTryFuture , ActorTryFutureExt , WrapFuture , WrapStream , } , handler :: { ActorResponse , AtomicResponse , Handler , Message , MessageResult , Response , ResponseActFuture , ResponseFuture , } , io , registry :: { ArbiterService , SystemService } , stream :: StreamHandler , supervisor :: Supervisor , sync :: { SyncArbiter , SyncContext } , utils :: { IntervalFunc , TimerFunc } , } ; }
};
}
