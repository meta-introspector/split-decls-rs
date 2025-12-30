// Generated macro for impl_650 (impl)
macro_rules! Depcrate_personnames_apiimpl_650 {
() => {
// Module: crate::personnames::api
// Provides: {"impl_650"}
// Dependencies: {}
impl fmt :: Debug for FieldModifierSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut debug = f . debug_struct ("FieldModifierSet") ; debug . field ("core" , & self . has_field (FieldModifier :: Core)) ; debug . field ("informal" , & self . has_field (FieldModifier :: Informal)) ; debug . field ("monogram" , & self . has_field (FieldModifier :: Monogram)) ; debug . field ("initial" , & self . has_field (FieldModifier :: Initial)) ; debug . field ("prefix" , & self . has_field (FieldModifier :: Prefix)) ; debug . field ("all_caps" , & self . has_field (FieldModifier :: AllCaps)) ; debug . field ("initial_cap" , & self . has_field (FieldModifier :: InitialCap)) ; debug . finish () } }
};
}
