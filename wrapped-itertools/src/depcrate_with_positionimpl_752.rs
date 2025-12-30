// Generated macro for impl_752 (impl)
macro_rules! Depcrate_with_positionimpl_752 {
() => {
// Module: crate::with_position
// Provides: {"impl_752"}
// Dependencies: {}
impl < I : Iterator > Iterator for WithPosition < I > { type Item = (Position , I :: Item) ; fn next (& mut self) -> Option < Self :: Item > { let item = self . peekable . next () ? ; let is_last = self . peekable . peek () . is_none () ; let is_first = ! self . handled_first ; self . handled_first = true ; Some ((Position { is_first , is_last } , item)) } fn size_hint (& self) -> (usize , Option < usize >) { self . peekable . size_hint () } fn fold < B , F > (mut self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { if let Some (mut head) = self . peekable . next () { if ! self . handled_first { match self . peekable . next () { Some (second) => { let first = std :: mem :: replace (& mut head , second) ; let position = Position { is_first : true , is_last : false , } ; init = f (init , (position , first)) ; } None => { let position = Position { is_first : true , is_last : true , } ; return f (init , (position , head)) ; } } } init = self . peekable . fold (init , | acc , mut item | { std :: mem :: swap (& mut head , & mut item) ; let position = Position { is_first : false , is_last : false , } ; f (acc , (position , item)) }) ; let position = Position { is_first : false , is_last : true , } ; init = f (init , (position , head)) ; } init } }
};
}
