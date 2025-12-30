// Generated macro for impl_125 (impl)
macro_rules! Depcrate_mapimpl_125 {
() => {
// Module: crate::map
// Provides: {"impl_125"}
// Dependencies: {}
impl < A , F , Req , Res > Service < Req > for Map < A , F , Req , Res > where A : Service < Req > , F : FnMut (A :: Response) -> Res + Clone , { type Response = Res ; type Error = A :: Error ; type Future = MapFuture < A , F , Req , Res > ; crate :: forward_ready ! (service) ; fn call (& self , req : Req) -> Self :: Future { MapFuture :: new (self . service . call (req) , self . f . clone ()) } }
};
}
