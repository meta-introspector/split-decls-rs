// Generated macro for impl_233 (impl)
macro_rules! Depcrate_rt_lazy_staticimpl_233 {
() => {
// Module: crate::rt::lazy_static
// Provides: {"impl_233"}
// Dependencies: {}
impl Set { # [doc = " Create an empty statics set."] pub (crate) fn new () -> Set { Set { statics : Some (HashMap :: new ()) , } } pub (crate) fn reset (& mut self) { assert ! (self . statics . is_none () , "lazy_static was not dropped during execution") ; self . statics = Some (HashMap :: new ()) ; } pub (crate) fn drop (& mut self) -> HashMap < StaticKeyId , StaticValue > { self . statics . take () . expect ("lazy_statics were dropped twice in one execution") } pub (crate) fn get_static < T : 'static > (& mut self , key : & 'static crate :: lazy_static :: Lazy < T > ,) -> Option < & mut StaticValue > { self . statics . as_mut () . expect ("attempted to access lazy_static during shutdown") . get_mut (& StaticKeyId :: new (key)) } pub (crate) fn init_static < T : 'static > (& mut self , key : & 'static crate :: lazy_static :: Lazy < T > , value : StaticValue ,) -> & mut StaticValue { let v = self . statics . as_mut () . expect ("attempted to access lazy_static during shutdown") . entry (StaticKeyId :: new (key)) ; if let std :: collections :: hash_map :: Entry :: Occupied (_) = v { unreachable ! ("told to init static, but it was already init'd") ; } v . or_insert (value) } }
};
}
