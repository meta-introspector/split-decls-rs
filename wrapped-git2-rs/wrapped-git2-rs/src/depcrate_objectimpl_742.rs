// Generated macro for impl_742 (impl)
macro_rules! Depcrate_objectimpl_742 {
() => {
// Module: crate::object
// Provides: {"impl_742"}
// Dependencies: {}
impl < 'repo > CastOrPanic for Object < 'repo > { fn cast_or_panic < T > (self , kind : ObjectType) -> T { assert_eq ! (mem :: size_of_val (& self) , mem :: size_of ::< T > ()) ; if self . kind () == Some (kind) { unsafe { let other = ptr :: read (& self as * const _ as * const T) ; mem :: forget (self) ; other } } else { let buf ; let akind = match self . kind () { Some (akind) => akind . str () , None => { buf = format ! ("unknown ({})" , unsafe { raw :: git_object_type (&* self . raw) }) ; & buf } } ; panic ! ("Expected object {} to be {} but it is {}" , self . id () , kind . str () , akind) } } }
};
}
