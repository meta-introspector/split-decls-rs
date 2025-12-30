// Generated macro for impl_651 (impl)
macro_rules! Depcrate_map_extimpl_651 {
() => {
// Module: crate::map_ext
// Provides: {"impl_651"}
// Dependencies: {}
impl Extend < HeaderValue > for ToValues < '_ > { fn extend < T : IntoIterator < Item = HeaderValue > > (& mut self , iter : T) { for value in iter { let entry = match :: std :: mem :: replace (& mut self . state , State :: Tmp) { State :: First (http :: header :: Entry :: Occupied (mut e)) => { e . insert (value) ; e } State :: First (http :: header :: Entry :: Vacant (e)) => e . insert_entry (value) , State :: Latter (mut e) => { e . append (value) ; e } State :: Tmp => unreachable ! ("ToValues State::Tmp") , } ; self . state = State :: Latter (entry) ; } } }
};
}
