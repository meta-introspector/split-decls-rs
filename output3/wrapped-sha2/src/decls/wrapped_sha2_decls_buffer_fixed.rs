use serde::{Deserialize, Serialize};
use std::collections::HashMap;
digest::buffer_fixed!(
    #[doc = " SHA-512/256 hasher."] pub struct Sha512_256(CtOutWrapper <
    block_api::Sha512VarCore, U32 >); oid : "2.16.840.1.101.3.4.2.6"; impl :
    FixedHashTraits;
);
