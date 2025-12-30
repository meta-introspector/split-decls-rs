// Generated macro for decomposing_normalize_to (macro)
macro_rules! Depcratedecomposing_normalize_to {
() => {
// Module: crate
// Provides: {"decomposing_normalize_to"}
// Dependencies: {}
macro_rules ! decomposing_normalize_to { ($ (# [$ meta : meta]) *, $ normalize_to : ident , $ write : path , $ slice : ty , $ prolog : block , $ as_slice : ident , $ fast : block , $ text : ident , $ sink : ident , $ decomposition : ident , $ decomposition_passthrough_bound : ident , $ undecomposed_starter : ident , $ pending_slice : ident , $ outer : lifetime ,) => { $ (# [$ meta]) * pub fn $ normalize_to < W : $ write + ? Sized > (& self , $ text : $ slice , $ sink : & mut W ,) -> core :: fmt :: Result { $ prolog let mut $ decomposition = self . normalize_iter ($ text . chars ()) ; debug_assert_eq ! ($ decomposition . ignorable_behavior , IgnorableBehavior :: Unsupported) ; let $ decomposition_passthrough_bound = $ decomposition . decomposition_passthrough_bound ; $ outer : loop { for cc in $ decomposition . buffer . drain (..) { $ sink . write_char (cc . character ()) ?; } debug_assert_eq ! ($ decomposition . buffer_pos , 0) ; let mut $ undecomposed_starter = if let Some (pending) = $ decomposition . pending . take () { pending } else { return Ok (()) ; } ; if $ undecomposed_starter . starter_and_decomposes_to_self () { $ sink . write_char ($ undecomposed_starter . character) ?; let $ pending_slice = $ decomposition . delegate .$ as_slice () ; $ fast } let starter = $ decomposition . decomposing_next ($ undecomposed_starter) ; $ sink . write_char (starter) ?; } } } ; }
};
}
