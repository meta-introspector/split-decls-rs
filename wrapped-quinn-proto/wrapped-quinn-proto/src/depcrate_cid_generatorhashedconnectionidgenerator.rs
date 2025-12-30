// Generated macro for HashedConnectionIdGenerator (struct)
macro_rules! Depcrate_cid_generatorHashedConnectionIdGenerator {
() => {
// Module: crate::cid_generator
// Provides: {"HashedConnectionIdGenerator"}
// Dependencies: {}
# [doc = " Generates 8-byte connection IDs that can be efficiently"] # [doc = " [`validate`](ConnectionIdGenerator::validate)d"] # [doc = ""] # [doc = " This generator uses a non-cryptographic hash and can therefore still be spoofed, but nonetheless"] # [doc = " helps prevents Quinn from responding to non-QUIC packets at very low cost."] pub struct HashedConnectionIdGenerator { key : u64 , lifetime : Option < Duration > , }
};
}
