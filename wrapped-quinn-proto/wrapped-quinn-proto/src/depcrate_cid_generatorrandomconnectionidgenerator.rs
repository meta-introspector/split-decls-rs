// Generated macro for RandomConnectionIdGenerator (struct)
macro_rules! Depcrate_cid_generatorRandomConnectionIdGenerator {
() => {
// Module: crate::cid_generator
// Provides: {"RandomConnectionIdGenerator"}
// Dependencies: {}
# [doc = " Generates purely random connection IDs of a specified length"] # [doc = ""] # [doc = " Random CIDs can be smaller than those produced by [`HashedConnectionIdGenerator`], but cannot be"] # [doc = " usefully [`validate`](ConnectionIdGenerator::validate)d."] # [derive (Debug , Clone , Copy)] pub struct RandomConnectionIdGenerator { cid_len : usize , lifetime : Option < Duration > , }
};
}
