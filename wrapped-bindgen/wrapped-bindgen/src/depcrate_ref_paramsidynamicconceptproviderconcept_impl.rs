// Generated macro for IDynamicConceptProviderConcept_Impl (trait)
macro_rules! Depcrate_ref_paramsIDynamicConceptProviderConcept_Impl {
() => {
// Module: crate::ref_params
// Provides: {"IDynamicConceptProviderConcept_Impl"}
// Dependencies: {}
pub trait IDynamicConceptProviderConcept_Impl : windows_core :: IUnknownImpl { fn GetConcept (& self , contextobject : windows_core :: Ref < IModelObject > , conceptid : * const windows_core :: GUID , conceptinterface : windows_core :: OutRef < windows_core :: IUnknown > , conceptmetadata : windows_core :: OutRef < IKeyStore > , hasconcept : * mut bool ,) -> windows_core :: Result < () > ; fn SetConcept (& self , contextobject : windows_core :: Ref < IModelObject > , conceptid : * const windows_core :: GUID , conceptinterface : windows_core :: Ref < windows_core :: IUnknown > , conceptmetadata : windows_core :: Ref < IKeyStore > ,) -> windows_core :: Result < () > ; fn NotifyParent (& self , parentmodel : windows_core :: Ref < IModelObject > ,) -> windows_core :: Result < () > ; fn NotifyParentChange (& self , parentmodel : windows_core :: Ref < IModelObject > ,) -> windows_core :: Result < () > ; fn NotifyDestruct (& self) -> windows_core :: Result < () > ; }
};
}
