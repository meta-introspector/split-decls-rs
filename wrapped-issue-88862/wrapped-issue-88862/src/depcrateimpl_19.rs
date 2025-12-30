// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < I , T , R > ServiceFactory < K > for Factory < I , T , R > where I : N < T , R > , R : std :: future :: Future , R :: Output : Responder , { type Future = std :: future :: Ready < Result < Self , () > > ; type Service = Self ; fn new_service (& self , _ : ()) -> Self :: Future { todo ! () } }
};
}
