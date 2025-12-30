// Generated macro for StartByteMap (struct)
macro_rules! Depcrate_util_startStartByteMap {
() => {
// Module: crate::util::start
// Provides: {"StartByteMap"}
// Dependencies: {}
# [doc = " A map from every possible byte value to its corresponding starting"] # [doc = " configuration."] # [doc = ""] # [doc = " This map is used in order to lookup the start configuration for a particular"] # [doc = " position in a haystack. This start configuration is then used in"] # [doc = " combination with things like the anchored mode and pattern ID to fully"] # [doc = " determine the start state."] # [doc = ""] # [doc = " Generally speaking, this map is only used for fully compiled DFAs and lazy"] # [doc = " DFAs. For NFAs (including the one-pass DFA), the start state is generally"] # [doc = " selected by virtue of traversing the NFA state graph. DFAs do the same"] # [doc = " thing, but at build time and not search time. (Well, technically the lazy"] # [doc = " DFA does it at search time, but it does enough work to cache the full"] # [doc = " result of the epsilon closure that the NFA engines tend to need to do.)"] # [derive (Clone)] pub (crate) struct StartByteMap { map : [Start ; 256] , }
};
}
