// Generated macro for impl_130 (impl)
macro_rules! Depcrate_serimpl_130 {
() => {
// Module: crate::ser
// Provides: {"impl_130"}
// Dependencies: {}
impl < 'a , W > StructSerializer < 'a , W > where W : Write , { # [inline] fn serialize_field_inner < T > (& mut self , key : & 'static str , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { if self . ser . packed { self . idx . serialize (& mut * self . ser) ? ; } else { key . serialize (& mut * self . ser) ? ; } value . serialize (& mut * self . ser) ? ; self . idx += 1 ; Ok (()) } # [inline] fn skip_field_inner (& mut self , _ : & 'static str) -> Result < () > { self . idx += 1 ; Ok (()) } # [inline] fn end_inner (self) -> Result < () > { Ok (()) } }
};
}
