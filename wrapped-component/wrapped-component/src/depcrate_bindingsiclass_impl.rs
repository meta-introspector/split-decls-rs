// Generated macro for IClass_Impl (trait)
macro_rules! Depcrate_bindingsIClass_Impl {
() => {
// Module: crate::bindings
// Provides: {"IClass_Impl"}
// Dependencies: {}
pub trait IClass_Impl : windows_core :: IUnknownImpl { fn Property (& self) -> windows_core :: Result < i32 > ; fn SetProperty (& self , value : i32) -> windows_core :: Result < () > ; fn Flags (& self) -> windows_core :: Result < Flags > ; fn Int32Array (& self , a : & [i32] , b : & mut [i32] , c : & mut windows_core :: Array < i32 > ,) -> windows_core :: Result < windows_core :: Array < i32 > > ; fn StringArray (& self , a : & [windows_core :: HSTRING] , b : & mut [windows_core :: HSTRING] , c : & mut windows_core :: Array < windows_core :: HSTRING > ,) -> windows_core :: Result < windows_core :: Array < windows_core :: HSTRING > > ; fn Input (& self , a : windows_core :: Ref < windows_core :: IInspectable > , b : windows_core :: Ref < Class > , c : windows_core :: Ref < windows :: Foundation :: IStringable > , d : windows_core :: Ref < Callback > ,) -> windows_core :: Result < () > ; }
};
}
