// Generated macro for impl_222 (impl)
macro_rules! Depcrate_pack_receiveimpl_222 {
() => {
// Module: crate::pack::receive
// Provides: {"impl_222"}
// Dependencies: {}
impl gix :: protocol :: fetch :: Negotiate for Negotiate < '_ > { fn mark_complete_and_common_ref (& mut self) -> Result < negotiate :: Action , negotiate :: Error > { Ok (negotiate :: Action :: MustNegotiate { remote_ref_target_known : vec ! [] , }) } fn add_wants (& mut self , arguments : & mut Arguments , _remote_ref_target_known : & [bool]) -> bool { let mut has_want = false ; for id in self . refmap . mappings . iter () . filter_map (| m | m . remote . as_id ()) { arguments . want (id) ; has_want = true ; } has_want } fn one_round (& mut self , _state : & mut negotiate :: one_round :: State , _arguments : & mut Arguments , _previous_response : Option < & Response > ,) -> Result < (negotiate :: Round , bool) , negotiate :: Error > { Ok ((negotiate :: Round { haves_sent : 0 , in_vain : 0 , haves_to_send : 0 , previous_response_had_at_least_one_in_common : false , } , true ,)) } }
};
}
