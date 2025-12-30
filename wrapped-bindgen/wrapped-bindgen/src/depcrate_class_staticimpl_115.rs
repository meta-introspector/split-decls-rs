// Generated macro for impl_115 (impl)
macro_rules! Depcrate_class_staticimpl_115 {
() => {
// Module: crate::class_static
// Provides: {"impl_115"}
// Dependencies: {}
impl GuidHelper { pub fn CreateNewGuid () -> windows_core :: Result < windows_core :: GUID > { Self :: IGuidHelperStatics (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . CreateNewGuid) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) }) } pub fn Empty () -> windows_core :: Result < windows_core :: GUID > { Self :: IGuidHelperStatics (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Empty) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) }) } pub fn Equals (target : windows_core :: GUID , value : windows_core :: GUID ,) -> windows_core :: Result < bool > { Self :: IGuidHelperStatics (| this | unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Equals) (windows_core :: Interface :: as_raw (this) , & target , & value , & mut result__ ,) . map (| | result__) }) } fn IGuidHelperStatics < R , F : FnOnce (& IGuidHelperStatics) -> windows_core :: Result < R > > (callback : F ,) -> windows_core :: Result < R > { static SHARED : windows_core :: imp :: FactoryCache < GuidHelper , IGuidHelperStatics > = windows_core :: imp :: FactoryCache :: new () ; SHARED . call (callback) } }
};
}
