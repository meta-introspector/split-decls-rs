// Generated macro for dev (module)
macro_rules! Depcratedev {
() => {
// Module: crate
// Provides: {"dev"}
// Dependencies: {}
pub mod dev { # ! [doc = " The `actix` prelude for library developers."] # ! [doc = ""] # ! [doc = " The purpose of this module is to alleviate imports of many common actix"] # ! [doc = " traits by adding a glob import to the top of actix heavy modules:"] # ! [doc = ""] # ! [doc = " ```"] # ! [doc = " # #![allow(unused_imports)]"] # ! [doc = " use actix::dev::*;"] # ! [doc = " ```"] pub use crate :: { address :: { Envelope , EnvelopeProxy , RecipientRequest , Request , ToEnvelope } , prelude :: * , } ; pub mod channel { pub use crate :: address :: channel :: { channel , AddressReceiver , AddressSender } ; } pub use crate :: { context_impl :: { AsyncContextParts , ContextFut , ContextParts } , handler :: { MessageResponse , OneshotSender } , mailbox :: Mailbox , registry :: { Registry , SystemRegistry } , } ; }
};
}
