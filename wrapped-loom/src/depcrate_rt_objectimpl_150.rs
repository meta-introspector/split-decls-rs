// Generated macro for impl_150 (impl)
macro_rules! Depcrate_rt_objectimpl_150 {
() => {
// Module: crate::rt::object
// Provides: {"impl_150"}
// Dependencies: {}
impl < T : Object < Entry = Entry > > Ref < T > { pub (super) fn branch_acquire (self , is_locked : bool , location : Location) { super :: branch (| execution | { trace ! (obj = ? self , ? is_locked , "Object::branch_acquire") ; self . set_action (execution , Action :: Opaque , location) ; if is_locked { execution . threads . active_mut () . set_blocked (location) ; } }) } pub (super) fn branch_action (self , action : impl Into < Action > + std :: fmt :: Debug , location : Location ,) { super :: branch (| execution | { trace ! (obj = ? self , ? action , "Object::branch_action") ; self . set_action (execution , action . into () , location) ; }) } pub (super) fn branch_disable (self , action : impl Into < Action > + std :: fmt :: Debug , disable : bool , location : Location ,) { super :: branch (| execution | { trace ! (obj = ? self , ? action , ? disable , "Object::branch_disable") ; self . set_action (execution , action . into () , location) ; if disable { execution . threads . active_mut () . set_blocked (location) ; } }) } pub (super) fn branch_opaque (self , location : Location) { self . branch_action (Action :: Opaque , location) } fn set_action (self , execution : & mut Execution , action : Action , location : Location) { assert ! (T :: get_ref (& execution . objects . entries [self . index]) . is_some () , "failed to get object for ref {:?}" , self) ; execution . threads . active_mut () . operation = Some (Operation { obj : self . erase () , action , location , }) ; } }
};
}
