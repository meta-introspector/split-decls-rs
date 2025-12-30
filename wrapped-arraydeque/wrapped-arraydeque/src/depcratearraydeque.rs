// Generated macro for ArrayDeque (struct)
macro_rules! DepcrateArrayDeque {
() => {
// Module: crate
// Provides: {"ArrayDeque"}
// Dependencies: {}
# [doc = " A fixed capacity ring buffer."] # [doc = ""] # [doc = " It can be stored directly on the stack if needed."] # [doc = ""] # [doc = " The \"default\" usage of this type as a queue is to use `push_back` to add to"] # [doc = " the queue, and `pop_front` to remove from the queue. Iterating over `ArrayDeque` goes front"] # [doc = " to back."] pub struct ArrayDeque < T , const CAP : usize , B : Behavior = Saturating > { xs : MaybeUninit < [T ; CAP] > , tail : usize , len : usize , marker : marker :: PhantomData < B > , }
};
}
