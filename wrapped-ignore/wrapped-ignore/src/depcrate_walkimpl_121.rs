// Generated macro for impl_121 (impl)
macro_rules! Depcrate_walkimpl_121 {
() => {
// Module: crate::walk
// Provides: {"impl_121"}
// Dependencies: {}
impl Stack { # [doc = " Create a work-stealing stack for each thread. The given messages"] # [doc = " correspond to the initial paths to start the search at. They will"] # [doc = " be distributed automatically to each stack in a round-robin fashion."] fn new_for_each_thread (threads : usize , init : Vec < Message >) -> Vec < Stack > { let deques : Vec < Deque < Message > > = std :: iter :: repeat_with (Deque :: new_lifo) . take (threads) . collect () ; let stealers = Arc :: < [Stealer < Message >] > :: from (deques . iter () . map (Deque :: stealer) . collect :: < Vec < _ > > () ,) ; let stacks : Vec < Stack > = deques . into_iter () . enumerate () . map (| (index , deque) | Stack { index , deque , stealers : stealers . clone () , }) . collect () ; init . into_iter () . rev () . zip (stacks . iter () . cycle ()) . for_each (| (m , s) | s . push (m)) ; stacks } # [doc = " Push a message."] fn push (& self , msg : Message) { self . deque . push (msg) ; } # [doc = " Pop a message."] fn pop (& self) -> Option < Message > { self . deque . pop () . or_else (| | self . steal ()) } # [doc = " Steal a message from another queue."] fn steal (& self) -> Option < Message > { let (left , right) = self . stealers . split_at (self . index) ; let right = & right [1 ..] ; right . iter () . chain (left . iter ()) . map (| s | s . steal_batch_and_pop (& self . deque)) . find_map (| s | s . success ()) } }
};
}
