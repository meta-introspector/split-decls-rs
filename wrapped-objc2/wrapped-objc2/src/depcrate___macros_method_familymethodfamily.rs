// Generated macro for MethodFamily (struct)
macro_rules! Depcrate___macros_method_familyMethodFamily {
() => {
// Module: crate::__macros::method_family
// Provides: {"MethodFamily"}
// Dependencies: {}
# [doc = " Helper for specifying the method family for a given selector."] # [doc = ""] # [doc = " Note that we can't actually check if a method is in a method family; only"] # [doc = " whether the _selector_ is in a _selector_ family."] # [doc = ""] # [doc = " The slight difference here is:"] # [doc = " - The method may be annotated with the `objc_method_family` attribute,"] # [doc = "   which would cause it to be in a different family. That this is not the"] # [doc = "   case is part of the `unsafe` contract of `msg_send!`."] # [doc = " - The method may not obey the added restrictions of the method family."] # [doc = "   The added restrictions are:"] # [doc = "   - `new`, `alloc`, `copy` and `mutableCopy`: The method must return a"] # [doc = "     retainable object pointer type - we ensure this by making"] # [doc = "     `message_send` return `Retained`."] # [doc = "   - `init`: The method must be an instance method and must return an"] # [doc = "     Objective-C pointer type - We ensure this by taking `Allocated<T>`,"] # [doc = "     which means it can't be a class method!"] # [doc = ""] # [doc = " <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retainable-object-pointers-as-operands-and-arguments>"] # [derive (Debug)] pub struct MethodFamily < const INNER : u8 > { }
};
}
