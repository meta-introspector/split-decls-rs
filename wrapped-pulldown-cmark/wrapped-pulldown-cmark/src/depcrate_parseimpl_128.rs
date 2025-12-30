// Generated macro for impl_128 (impl)
macro_rules! Depcrate_parseimpl_128 {
() => {
// Module: crate::parse
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'input , T > BrokenLinkCallback < 'input > for T where T : FnMut (BrokenLink < 'input >) -> Option < (CowStr < 'input > , CowStr < 'input >) > , { fn handle_broken_link (& mut self , link : BrokenLink < 'input > ,) -> Option < (CowStr < 'input > , CowStr < 'input >) > { self (link) } }
};
}
