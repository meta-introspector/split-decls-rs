// Generated macro for ContextElement (enum)
macro_rules! Depcrate_templateContextElement {
() => {
// Module: crate::template
// Provides: {"ContextElement"}
// Dependencies: {}
# [doc = " Enum defining the different kinds of records on the context stack."] enum ContextElement < 'render , 'template > { # [doc = " Object contexts shadow everything below them on the stack, because every name is looked up"] # [doc = " in this object."] Object (& 'render Value) , # [doc = " Named contexts shadow only one name. Any path that starts with that name is looked up in"] # [doc = " this object, and all others are passed on down the stack."] Named (& 'template str , & 'render Value) , # [doc = " Iteration contexts shadow one name with the current value of the iteration. They also"] # [doc = " store the iteration state. The two usizes are the index of the current value and the length"] # [doc = " of the array that we're iterating over."] Iteration (& 'template str , & 'render Value , usize , usize , slice :: Iter < 'render , Value > ,) , }
};
}
