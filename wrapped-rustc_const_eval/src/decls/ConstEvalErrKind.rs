macro_rules! ConstEvalErrKind {
    () => {
        # [doc = " The CTFE machine has some custom error kinds."] # [derive (Clone , Debug)] pub enum ConstEvalErrKind { ConstAccessesMutGlobal , ModifiedGlobal , RecursiveStatic , AssertFailure (AssertKind < ConstInt >) , Panic { msg : Symbol , line : u32 , col : u32 , file : Symbol , } , WriteThroughImmutablePointer , # [doc = " Called `const_make_global` twice."] ConstMakeGlobalPtrAlreadyMadeGlobal (AllocId) , # [doc = " Called `const_make_global` on a non-heap pointer."] ConstMakeGlobalPtrIsNonHeap (Pointer < Option < CtfeProvenance > >) , # [doc = " Called `const_make_global` on a dangling pointer."] ConstMakeGlobalWithDanglingPtr (Pointer < Option < CtfeProvenance > >) , # [doc = " Called `const_make_global` on a pointer that does not start at the"] # [doc = " beginning of an object."] ConstMakeGlobalWithOffset (Pointer < Option < CtfeProvenance > >) , }
    };
}

ConstEvalErrKind!()