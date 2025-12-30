// Generated macro for impl_112 (impl)
macro_rules! Depcrate_kv_valueimpl_112 {
() => {
// Module: crate::kv::value
// Provides: {"impl_112"}
// Dependencies: {}
impl < 'a , 'v , T : ? Sized > VisitValue < 'v > for & 'a mut T where T : VisitValue < 'v > , { fn visit_any (& mut self , value : Value) -> Result < () , Error > { (* * self) . visit_any (value) } fn visit_null (& mut self) -> Result < () , Error > { (* * self) . visit_null () } fn visit_u64 (& mut self , value : u64) -> Result < () , Error > { (* * self) . visit_u64 (value) } fn visit_i64 (& mut self , value : i64) -> Result < () , Error > { (* * self) . visit_i64 (value) } fn visit_u128 (& mut self , value : u128) -> Result < () , Error > { (* * self) . visit_u128 (value) } fn visit_i128 (& mut self , value : i128) -> Result < () , Error > { (* * self) . visit_i128 (value) } fn visit_f64 (& mut self , value : f64) -> Result < () , Error > { (* * self) . visit_f64 (value) } fn visit_bool (& mut self , value : bool) -> Result < () , Error > { (* * self) . visit_bool (value) } fn visit_str (& mut self , value : & str) -> Result < () , Error > { (* * self) . visit_str (value) } fn visit_borrowed_str (& mut self , value : & 'v str) -> Result < () , Error > { (* * self) . visit_borrowed_str (value) } fn visit_char (& mut self , value : char) -> Result < () , Error > { (* * self) . visit_char (value) } # [cfg (feature = "kv_std")] fn visit_error (& mut self , err : & (dyn std :: error :: Error + 'static)) -> Result < () , Error > { (* * self) . visit_error (err) } # [cfg (feature = "kv_std")] fn visit_borrowed_error (& mut self , err : & 'v (dyn std :: error :: Error + 'static) ,) -> Result < () , Error > { (* * self) . visit_borrowed_error (err) } }
};
}
