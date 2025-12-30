// Generated macro for dynamic_cast (function)
macro_rules! Depcrate_com_objectdynamic_cast {
() => {
// Module: crate::com_object
// Provides: {"dynamic_cast"}
// Dependencies: {}
# [test] fn dynamic_cast () { let app = MyApp :: new (42) ; let unknown = app . to_interface :: < IUnknown > () ; assert ! (! unknown . is_object ::< SendableThing > ()) ; assert ! (unknown . is_object ::< MyApp > ()) ; let dyn_app_ref : & MyApp_Impl = unknown . cast_object_ref :: < MyApp > () . unwrap () ; assert_eq ! (dyn_app_ref . signature , APP_SIGNATURE) ; let dyn_app_owned : ComObject < MyApp > = unknown . cast_object () . unwrap () ; assert_eq ! (dyn_app_owned . signature , APP_SIGNATURE) ; let dyn_app_owned_2 : ComObject < MyApp > = ComObject :: cast_from (& unknown) . unwrap () ; assert_eq ! (dyn_app_owned_2 . signature , APP_SIGNATURE) ; }
};
}
