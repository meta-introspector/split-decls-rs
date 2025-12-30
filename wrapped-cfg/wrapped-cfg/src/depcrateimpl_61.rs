// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl fmt :: Debug for CfgOptions { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut items = self . enabled . iter () . map (| atom | match atom { CfgAtom :: Flag (it) => it . to_string () , CfgAtom :: KeyValue { key , value } => format ! ("{key}={value}") , }) . collect :: < Vec < _ > > () ; items . sort () ; f . debug_tuple ("CfgOptions") . field (& items) . finish () } }
};
}
