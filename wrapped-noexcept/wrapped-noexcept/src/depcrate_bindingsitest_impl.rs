// Generated macro for ITest_Impl (trait)
macro_rules! Depcrate_bindingsITest_Impl {
() => {
// Module: crate::bindings
// Provides: {"ITest_Impl"}
// Dependencies: {}
pub trait ITest_Impl : windows_core :: IUnknownImpl { fn MethodString (& self , test : & windows_core :: HSTRING) -> windows_core :: Result < () > ; fn MethodInt32 (& self , test : i32) -> windows_core :: Result < () > ; fn MethodTest (& self , test : windows_core :: Ref < ITest >) -> windows_core :: Result < () > ; fn String (& self) -> windows_core :: Result < windows_core :: HSTRING > ; fn SetString (& self , value : & windows_core :: HSTRING) -> windows_core :: Result < () > ; fn Int32 (& self) -> windows_core :: Result < i32 > ; fn SetInt32 (& self , value : i32) -> windows_core :: Result < () > ; fn Test (& self) -> windows_core :: Result < ITest > ; fn SetTest (& self , value : windows_core :: Ref < ITest >) -> windows_core :: Result < () > ; fn MethodStringN (& self , test : & windows_core :: HSTRING) ; fn MethodInt32N (& self , test : i32) ; fn MethodTestN (& self , test : windows_core :: Ref < ITest >) ; fn StringN (& self) -> windows_core :: HSTRING ; fn SetStringN (& self , value : & windows_core :: HSTRING) ; fn Int32N (& self) -> i32 ; fn SetInt32N (& self , value : i32) ; fn TestN (& self) -> Option < ITest > ; fn SetTestN (& self , value : windows_core :: Ref < ITest >) ; }
};
}
