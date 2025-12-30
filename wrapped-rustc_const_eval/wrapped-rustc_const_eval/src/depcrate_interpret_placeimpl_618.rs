// Generated macro for impl_618 (impl)
macro_rules! Depcrate_interpret_placeimpl_618 {
() => {
// Module: crate::interpret::place
// Provides: {"impl_618"}
// Dependencies: {}
impl < Prov : Provenance > std :: fmt :: Debug for MPlaceTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("MPlaceTy") . field ("mplace" , & self . mplace) . field ("ty" , & format_args ! ("{}" , self . layout . ty)) . finish () } }
};
}
