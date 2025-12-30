// Generated macro for IKeyStore_Impl (trait)
macro_rules! Depcrate_ref_paramsIKeyStore_Impl {
() => {
// Module: crate::ref_params
// Provides: {"IKeyStore_Impl"}
// Dependencies: {}
pub trait IKeyStore_Impl : windows_core :: IUnknownImpl { fn GetKey (& self , key : & windows_core :: PCWSTR , object : windows_core :: OutRef < IModelObject > , metadata : windows_core :: OutRef < IKeyStore > ,) -> windows_core :: Result < () > ; fn SetKey (& self , key : & windows_core :: PCWSTR , object : windows_core :: Ref < IModelObject > , metadata : windows_core :: Ref < IKeyStore > ,) -> windows_core :: Result < () > ; fn GetKeyValue (& self , key : & windows_core :: PCWSTR , object : windows_core :: OutRef < IModelObject > , metadata : windows_core :: OutRef < IKeyStore > ,) -> windows_core :: Result < () > ; fn SetKeyValue (& self , key : & windows_core :: PCWSTR , object : windows_core :: Ref < IModelObject > ,) -> windows_core :: Result < () > ; fn ClearKeys (& self) -> windows_core :: Result < () > ; }
};
}
