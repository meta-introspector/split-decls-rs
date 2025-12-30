// Generated macro for non_static_type_id (function)
macro_rules! Depcrate_utilsnon_static_type_id {
() => {
// Module: crate::utils
// Provides: {"non_static_type_id"}
// Dependencies: {}
# [doc = " Produces type IDs that are compatible with `TypeId::of::<T>`, but without"] # [doc = " `T: 'static` bound."] fn non_static_type_id < T : ? Sized > () -> TypeId { trait NonStaticAny { fn get_type_id (& self) -> TypeId where Self : 'static ; } impl < T : ? Sized > NonStaticAny for PhantomData < T > { fn get_type_id (& self) -> TypeId where Self : 'static , { TypeId :: of :: < T > () } } let phantom_data = PhantomData :: < T > ; NonStaticAny :: get_type_id (unsafe { mem :: transmute :: < & dyn NonStaticAny , & (dyn NonStaticAny + 'static) > (& phantom_data) }) }
};
}
