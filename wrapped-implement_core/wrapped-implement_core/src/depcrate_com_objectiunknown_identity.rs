// Generated macro for iunknown_identity (function)
macro_rules! Depcrate_com_objectiunknown_identity {
() => {
// Module: crate::com_object
// Provides: {"iunknown_identity"}
// Dependencies: {}
# [test] fn iunknown_identity () { let app = MyApp :: new (0) ; let iunknown : IUnknown = app . to_interface () ; let _ifoo : IFoo = app . to_interface () ; let ibar : IBar = app . to_interface () ; let ibar2 : IBar2 = app . to_interface () ; { let ibar_iunknown_static : IUnknown = (* ibar) . clone () ; assert_ne ! (ibar_iunknown_static . as_raw () , iunknown . as_raw () , "IBar-to-IUnknown is non-canonical interface chain") ; assert_eq ! (ibar_iunknown_static , iunknown , "QueryInterface for IUnknown yields same pointer") ; } let ibar2_ibar : & IBar = & ibar2 ; assert_ne ! (ibar . as_raw () , ibar2_ibar . as_raw () , "IBar from different interface chains have different pointer values") ; assert_eq ! (ibar , * ibar2_ibar , "IBar from different interface chains are equal (using QueryInterface)") ; }
};
}
