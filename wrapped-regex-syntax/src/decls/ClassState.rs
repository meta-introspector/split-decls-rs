macro_rules! deps {
    () => {
        ClassBracketed!();
        ClassSetBinaryOpKind!();
        ClassSet!();
        ClassSetUnion!();
    };
}

macro_rules! ClassState {
    () => {
        deps!();
        # [doc = " ClassState represents a single stack frame while parsing character classes."] # [doc = " Each frame records the state up to an intersection, difference, symmetric"] # [doc = " difference or nested class."] # [doc = ""] # [doc = " Note that a parser's character class stack is only non-empty when parsing"] # [doc = " a character class. In all other cases, it is empty."] # [derive (Clone , Debug)] enum ClassState { # [doc = " This state is pushed whenever an opening bracket is found."] Open { # [doc = " The union of class items immediately preceding this class."] union : ast :: ClassSetUnion , # [doc = " The class that has been opened. Typically this just corresponds"] # [doc = " to the `[`, but it can also include `[^` since `^` indicates"] # [doc = " negation of the class."] set : ast :: ClassBracketed , } , # [doc = " This state is pushed when a operator is seen. When popped, the stored"] # [doc = " set becomes the left hand side of the operator."] Op { # [doc = " The type of the operation, i.e., &&, -- or ~~."] kind : ast :: ClassSetBinaryOpKind , # [doc = " The left-hand side of the operator."] lhs : ast :: ClassSet , } , }
    };
}

ClassState!()