// Generated macro for impl_129 (impl)
macro_rules! Depcrate_parseimpl_129 {
() => {
// Module: crate::parse
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'input > BrokenLinkCallback < 'input > for Box < dyn BrokenLinkCallback < 'input > > { fn handle_broken_link (& mut self , link : BrokenLink < 'input > ,) -> Option < (CowStr < 'input > , CowStr < 'input >) > { (* * self) . handle_broken_link (link) } }
};
}
