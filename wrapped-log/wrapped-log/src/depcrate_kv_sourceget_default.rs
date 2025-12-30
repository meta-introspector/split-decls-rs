// Generated macro for get_default (function)
macro_rules! Depcrate_kv_sourceget_default {
() => {
// Module: crate::kv::source
// Provides: {"get_default"}
// Dependencies: {}
# [doc = " The default implementation of `Source::get`"] fn get_default < 'v > (source : & 'v (impl Source + ? Sized) , key : Key) -> Option < Value < 'v > > { struct Get < 'k , 'v > { key : Key < 'k > , found : Option < Value < 'v > > , } impl < 'k , 'kvs > VisitSource < 'kvs > for Get < 'k , 'kvs > { fn visit_pair (& mut self , key : Key < 'kvs > , value : Value < 'kvs >) -> Result < () , Error > { if self . key == key { self . found = Some (value) ; } Ok (()) } } let mut get = Get { key , found : None } ; let _ = source . visit (& mut get) ; get . found }
};
}
