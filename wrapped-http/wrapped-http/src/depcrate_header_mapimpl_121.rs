// Generated macro for impl_121 (impl)
macro_rules! Depcrate_header_mapimpl_121 {
() => {
// Module: crate::header::map
// Provides: {"impl_121"}
// Dependencies: {}
impl Pos { # [inline] fn new (index : usize , hash : HashValue) -> Self { debug_assert ! (index < MAX_SIZE) ; Pos { index : index as Size , hash , } } # [inline] fn none () -> Self { Pos { index : ! 0 , hash : HashValue (0) , } } # [inline] fn is_some (& self) -> bool { ! self . is_none () } # [inline] fn is_none (& self) -> bool { self . index == ! 0 } # [inline] fn resolve (& self) -> Option < (usize , HashValue) > { if self . is_some () { Some ((self . index as usize , self . hash)) } else { None } } }
};
}
