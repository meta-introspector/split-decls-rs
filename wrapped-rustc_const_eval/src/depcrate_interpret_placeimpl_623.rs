// Generated macro for impl_623 (impl)
macro_rules! Depcrate_interpret_placeimpl_623 {
() => {
// Module: crate::interpret::place
// Provides: {"impl_623"}
// Dependencies: {}
impl < Prov : Provenance > std :: fmt :: Debug for PlaceTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("PlaceTy") . field ("place" , & self . place) . field ("ty" , & format_args ! ("{}" , self . layout . ty)) . finish () } }
};
}
