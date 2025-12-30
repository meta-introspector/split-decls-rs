// Generated macro for init (function)
macro_rules! Depcrate_sllinit {
() => {
// Module: crate::sll
// Provides: {"init"}
// Dependencies: {}
# [cold] pub (crate) fn init < 'a , E : Elem > (head : Option < & 'a Cell < * const E > > , elem : & E ,) -> AddToSllResult < 'a , E > { if let Some (head) = head { link (head , elem) } else { AddToSllResult :: NoHead } }
};
}
