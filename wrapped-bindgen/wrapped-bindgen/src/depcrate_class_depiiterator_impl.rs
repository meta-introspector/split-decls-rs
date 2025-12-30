// Generated macro for IIterator_Impl (trait)
macro_rules! Depcrate_class_depIIterator_Impl {
() => {
// Module: crate::class_dep
// Provides: {"IIterator_Impl"}
// Dependencies: {}
pub trait IIterator_Impl < T > : windows_core :: IUnknownImpl where T : windows_core :: RuntimeType + 'static , { fn Current (& self) -> windows_core :: Result < T > ; fn HasCurrent (& self) -> windows_core :: Result < bool > ; fn MoveNext (& self) -> windows_core :: Result < bool > ; fn GetMany (& self , items : & mut [< T as windows_core :: Type < T > > :: Default] ,) -> windows_core :: Result < u32 > ; }
};
}
